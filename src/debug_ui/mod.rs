mod capped_dequeue_vec;
mod inspect;
pub mod renderer;

use crate::game::camera::Camera;

use crate::debug_ui::capped_dequeue_vec::CappedVecDeque;
use crate::game::player::Player;
use bevy_ecs::prelude::World;
use egui::emath::Numeric;
use egui::plot::{Line, Plot, PlotPoints};
use egui::{ClippedPrimitive, Color32, FontDefinitions, FullOutput, Style, TexturesDelta, Visuals};
use egui_winit_platform::{Platform, PlatformDescriptor};
use std::time::Duration;
use wgpu::RenderPass;
use winit::dpi::PhysicalSize;
use winit::event::Event;
use winit::window::Window;

pub struct DebugUI {
    pub platform: Platform,

    resp: CappedVecDeque<f64>,
}

impl DebugUI {
    pub fn new(window: &Window) -> Self {
        let PhysicalSize { width, height } = window.inner_size();

        let mut platform = Platform::new(PlatformDescriptor {
            physical_width: width,
            physical_height: height,
            scale_factor: window.scale_factor(),
            font_definitions: FontDefinitions::default(),
            style: Default::default(),
        });

        Self {
            platform,
            resp: CappedVecDeque::new(500),
        }
    }

    pub fn render(&mut self, _render_pass: &mut RenderPass) {}

    pub fn record_event<T>(&mut self, event: &Event<'_, T>) {
        self.platform.handle_event(event);
    }

    pub fn update(&mut self, world: &World, window: &Window, time_elapsed: &Duration) {
        let time_elapsed = time_elapsed.as_secs_f64();
        self.platform.update_time(time_elapsed);
        self.platform.begin_frame();

        let ctx = self.platform.context();

        let player = world.get_resource::<Player>().unwrap();
        let camera = world.get_resource::<Camera>().unwrap();
        let (camera_yaw, camera_pitch) = camera.get_yaw_pitch();

        ctx.set_style(Style {
            visuals: Visuals {
                window_fill: Color32::from_black_alpha(20),
                ..Default::default()
            },
            ..Default::default()
        });

        self.resp.push_back(time_elapsed);

        egui::Window::new("Inspector")
            .resizable(false)
            .default_width(300.0)
            .show(&ctx, |ui| {
                egui::trace!(ui);
                ui.vertical_centered(|ui| {
                    ui.heading("Inspect");
                });

                ui.separator();

                ui.label(format!("Yaw: {:.1} Pitch: {:.1}", camera_yaw, camera_pitch));
                ui.label(format!("Direction: {}", camera.get_direction()));
                ui.label(format!("Pos: {}", camera.position));
                let mut a = player.flying;
                ui.checkbox(&mut a, "Fly");

                ui.separator();

                Plot::new("FPS").show(ui, |plot_ui| {
                    let l = self
                        .resp
                        .iter()
                        .enumerate()
                        .map(|(a, b)| [a.to_f64(), *b])
                        .collect::<PlotPoints>();
                    plot_ui.line(Line::new(l));
                })
            });

        // {
        //     ui.window("Info")
        //         .size([300.0, 300.0], Condition::FirstUseEver)
        //         .build(|| {
        //             ui.text(format!("DT: {}", time_elapsed.as_secs_f32()));
        //             ui.text("Camera");
        //             ui.separator();
        //             let mouse_pos = ui.io().mouse_pos;
        //
        //             ui.text(format!(
        //                 "Mouse Position: ({:.1},{:.1})",
        //                 mouse_pos[0], mouse_pos[1]
        //             ));
        //         });
        // }
    }

    pub fn get_draw_data(&mut self, window: &Window) -> DebugUIDrawData {
        let full_output = self.platform.end_frame(Some(window));
        let paint_jobs = self.platform.context().tessellate(full_output.shapes);

        DebugUIDrawData {
            textures_delta: full_output.textures_delta,
            paint_jobs,
        }
    }
}

pub struct DebugUIDrawData {
    pub textures_delta: TexturesDelta,
    pub paint_jobs: Vec<ClippedPrimitive>,
}
