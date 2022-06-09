use crate::renderer::context::RenderContext;
use imgui::{Condition, DrawData, FontSource};
use imgui_wgpu::{Renderer, RendererConfig};
use imgui_winit_support::WinitPlatform;
use legion::World;
use std::time::Duration;
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
            &window,
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

    pub fn render(&mut self, render_pass: &mut RenderPass) {}

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

        {
            let window = imgui::Window::new("Hello world");
            window
                .size([300.0, 100.0], Condition::FirstUseEver)
                .build(&ui, || {
                    ui.text("Hello world!");
                    ui.text("This...is...imgui-rs on WGPU!");
                    ui.separator();
                    let mouse_pos = ui.io().mouse_pos;
                    ui.text(format!(
                        "Mouse Position: ({:.1},{:.1})",
                        mouse_pos[0], mouse_pos[1]
                    ));
                });

            let window = imgui::Window::new("Hello too");
            window
                .size([400.0, 200.0], Condition::FirstUseEver)
                .position([400.0, 200.0], Condition::FirstUseEver)
                .build(&ui, || {
                    ui.text(format!("Frametime: {:?}", 123));
                });

            let mut demo_open = true;
            ui.show_demo_window(&mut demo_open);
        }

        self.platform.prepare_render(&ui, window);

        DebugUIRenderState {
            draw_data: ui.render(),
        }
    }
}

pub struct DebugUIRenderState<'ui> {
    pub(crate) draw_data: &'ui DrawData,
}
