use crate::game::debug_ui::{DebugUI, DebugUIRenderState};
use crate::renderer::context::RenderContext;
use crate::renderer::debug_ui_renderer::DebugUIRenderer;
use crate::renderer::game::GameRenderer;
use imgui::{Condition, FontSource};
use imgui_winit_support::WinitPlatform;
use std::rc::Rc;
use std::time::Duration;
use wgpu::{
    Color, CommandEncoderDescriptor, LoadOp, Operations, PresentMode, RenderPassColorAttachment,
    RenderPassDescriptor, TextureFormat, TextureViewDescriptor,
};
use winit::window::Window;

pub struct Renderer {
    pub render_context: RenderContext,

    pub game_renderer: GameRenderer,
    pub debug_ui_renderer: DebugUIRenderer,
}

impl Renderer {
    pub async fn new(window: Rc<Window>, debug_ui: &mut DebugUI) -> Self {
        let mut render_context = RenderContext::new(Rc::clone(&window)).await;
        let debug_ui_renderer = DebugUIRenderer::new(&mut render_context, debug_ui);

        Self {
            render_context,
            game_renderer: GameRenderer::new(),
            debug_ui_renderer,
        }
    }

    pub fn render(&mut self, debug_ui_render_state: &DebugUIRenderState, time_elapsed: &Duration) {
        self.render_context
            .render(&mut self.debug_ui_renderer, debug_ui_render_state);
        // self.render_context.render(|render_pass| {
        // self.debug_ui_renderer
        //     .render(debug_ui_render_state, &self.render_context, render_pass)
        //     .unwrap();
        // self.debug_ui_renderer
        //     .renderer
        //     .render(
        //         debug_ui_render_state.draw_data,
        //         &self.render_context.queue,
        //         &self.render_context.device,
        //         render_pass,
        //     )
        //     .unwrap();
        // });
    }
}
