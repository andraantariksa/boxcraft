use crate::game::camera::Camera;

use imgui::{Condition, DrawData, FontSource};

use imgui_winit_support::WinitPlatform;
use std::time::Duration;
use bevy_ecs::prelude::*;
use wgpu::RenderPass;
use winit::event::Event;
use winit::window::Window;

pub struct DebugUI {
    pub imgui: imgui::Context,
    platform: WinitPlatform,
}

impl DebugUI {
    pub fn new(window: &Window) -> Self {
        let hidpi_factor = window.scale_factor();

        let mut imgui = imgui::Context::create();
        let mut platform = WinitPlatform::init(&mut imgui);
        platform.attach_window(
            imgui.io_mut(),
            window,
            imgui_winit_support::HiDpiMode::Default,
        );
        imgui.set_ini_filename(None);

        let font_size = (13.0 * hidpi_factor) as f32;
        imgui.io_mut().font_global_scale = (1.0 / hidpi_factor) as f32;
        imgui.fonts().add_font(&[FontSource::DefaultFontData {
            config: Some(imgui::FontConfig {
                oversample_h: 1,
                pixel_snap_h: true,
                size_pixels: font_size,
                ..Default::default()
            }),
        }]);

        Self { imgui, platform }
    }

    pub fn render(&mut self, _render_pass: &mut RenderPass) {}

    pub fn record_event<T>(&mut self, window: &Window, event: &Event<'_, T>) {
        self.platform
            .handle_event(self.imgui.io_mut(), window, event);
    }

    pub fn update(
        &mut self,
        world: &World,
        window: &Window,
        time_elapsed: &Duration,
    ) -> DebugUIRenderState {
        self.imgui.io_mut().update_delta_time(*time_elapsed);
        self.platform
            .prepare_frame(self.imgui.io_mut(), window)
            .unwrap();
        let ui = self.imgui.frame();

        // self.profile_ui.window(&ui);

        let camera = world.get_resource::<Camera>().unwrap();
        let (camera_yaw, camera_pitch) = camera.get_yaw_pitch();

        {
            ui.window("Info")
                .size([300.0, 300.0], Condition::FirstUseEver)
                .build(|| {
                    ui.text(format!("DT: {}", time_elapsed.as_secs_f32()));
                    ui.text("Camera");
                    ui.separator();
                    let mouse_pos = ui.io().mouse_pos;

                    ui.text(format!(
                        "Mouse Position: ({:.1},{:.1})",
                        mouse_pos[0], mouse_pos[1]
                    ));
                    ui.text(format!("Yaw: {:.1} Pitch: {:.1}", camera_yaw, camera_pitch));
                    ui.text(format!("Direction: {}", camera.get_direction()));
                    ui.text(format!("Pos: {}", camera.position));
                });
        }

        self.platform.prepare_render(&ui, window);

        DebugUIRenderState {
            draw_data: self.imgui.render(),
        }
    }
}

pub struct DebugUIRenderState<'ui> {
    pub draw_data: &'ui DrawData,
}
