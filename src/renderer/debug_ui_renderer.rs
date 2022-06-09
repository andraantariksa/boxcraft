use crate::game::debug_ui::{DebugUI, DebugUIRenderState};
use crate::renderer::context::RenderContext;
use imgui::sys::ImGuiContext;
use imgui::{Context, DrawData};
use imgui_wgpu::{Renderer, RendererConfig, RendererResult};
use wgpu::RenderPass;

pub struct DebugUIRenderer {
    pub renderer: Renderer,
}

impl DebugUIRenderer {
    pub fn new(render_context: &mut RenderContext, debug_ui: &mut DebugUI) -> Self {
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

    // pub fn render(
    //     &mut self,
    //     render_state: &DebugUIRenderState,
    //     render_context: &RenderContext,
    //     render_pass: &mut RenderPass,
    // ) -> RendererResult<()> {
    //     self.renderer.render(
    //         render_state.draw_data,
    //         &render_context.queue,
    //         &render_context.device,
    //         render_pass,
    //     )
    // }
}
