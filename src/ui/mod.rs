pub mod renderer;

use crate::game::camera::Camera;


use crate::game::player::Player;
use bevy_ecs::prelude::*;


use egui::{
    ClippedPrimitive, Color32, Context, FontDefinitions, Style, TexturesDelta, Visuals,
};
use egui_winit_platform::{Platform, PlatformDescriptor};

use wgpu::RenderPass;
use winit::dpi::PhysicalSize;
use winit::event::Event;
use winit::window::Window;

#[derive(Resource)]
pub struct UI {
    pub platform: Platform,
    // resp: CappedVecDeque<f64>,
}

impl UI {
    pub fn new(window: &Window) -> Self {
        let PhysicalSize { width, height } = window.inner_size();

        let platform = Platform::new(PlatformDescriptor {
            physical_width: width,
            physical_height: height,
            scale_factor: window.scale_factor(),
            font_definitions: FontDefinitions::default(),
            style: Default::default(),
        });

        Self {
            platform,
            // resp: CappedVecDeque::new(500),
        }
    }

    pub fn render(&mut self, _render_pass: &mut RenderPass) {}

    pub fn record_event<T>(&mut self, event: &Event<'_, T>) {
        self.platform.handle_event(event);
    }

    pub fn pre_update(&mut self, time_elapsed: f64) {
        self.platform.update_time(time_elapsed);
        self.platform.begin_frame();
    }

    pub fn get_draw_data(&mut self, window: &Window) -> UIDrawData {
        let full_output = self.platform.end_frame(Some(window));
        let paint_jobs = self.platform.context().tessellate(full_output.shapes);

        UIDrawData {
            textures_delta: full_output.textures_delta,
            paint_jobs,
        }
    }

    pub fn context(&self) -> Context {
        self.platform.context()
    }
}

pub struct UIDrawData {
    pub textures_delta: TexturesDelta,
    pub paint_jobs: Vec<ClippedPrimitive>,
}

pub fn update_draw_ui(ui: Res<UI>, player: Res<Player>, camera: Res<Camera>) {
    let ctx = ui.context();

    let (camera_yaw, camera_pitch) = camera.get_yaw_pitch();

    ctx.set_style(Style {
        visuals: Visuals {
            window_fill: Color32::from_black_alpha(20),
            ..Default::default()
        },
        ..Default::default()
    });

    // self.resp.push_back(time_elapsed);

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

            // Plot::new("FPS").show(ui, |plot_ui| {
            //     let l = self
            //         .resp
            //         .iter()
            //         .enumerate()
            //         .map(|(a, b)| [a.to_f64(), *b])
            //         .collect::<PlotPoints>();
            //     plot_ui.line(Line::new(l));
            // })
        });
}
