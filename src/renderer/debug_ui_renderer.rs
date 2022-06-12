use crate::game::debug_ui::{DebugUI, DebugUIRenderState};
use crate::renderer::context::RenderContext;
use imgui::sys::ImGuiContext;
use imgui::{Context, DrawData};
use imgui_wgpu::{Renderer, RendererConfig, RendererResult};
use wgpu::RenderPass;
use winit::dpi::PhysicalSize;

pub struct DebugUIRenderer {
    pub renderer: Renderer,
}

impl DebugUIRenderer {
    pub fn new(render_context: &RenderContext, debug_ui: &mut DebugUI) -> Self {
        let mut renderer = Renderer::new(
            &mut debug_ui.imgui,
            &render_context.device,
            &render_context.queue,
            RendererConfig {
                texture_format: render_context.render_surface_config.format,
                ..Default::default()
            },
        );
        Self { renderer }
    }

    // pub fn resize(&mut self, new_window_size: &PhysicalSize<u32>) {}

    pub fn render<'r>(
        &'r mut self,
        render_context: &RenderContext,
        render_pass: &mut RenderPass<'r>,
        render_state: &DebugUIRenderState,
    ) -> RendererResult<()> {
        self.renderer.render(
            render_state.draw_data,
            &render_context.queue,
            &render_context.device,
            render_pass,
        )
    }
}
