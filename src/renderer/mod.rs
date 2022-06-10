use crate::game::camera::Camera;
use crate::game::debug_ui::{DebugUI, DebugUIRenderState};
use crate::renderer::camera::CameraRenderer;
use crate::renderer::context::RenderContext;
use crate::renderer::debug_ui_renderer::DebugUIRenderer;
use crate::renderer::game_renderer::GameRenderer;
use atomic_refcell::AtomicRef;
use imgui::{Condition, FontSource};
use imgui_winit_support::WinitPlatform;
use std::ops::Deref;
use std::rc::Rc;
use std::time::Duration;
use wgpu::{
    Color, CommandEncoderDescriptor, LoadOp, Operations, PresentMode, RenderPassColorAttachment,
    RenderPassDescriptor, TextureFormat, TextureViewDescriptor,
};
use winit::dpi::PhysicalSize;
use winit::window::Window;

pub mod block;
pub mod camera;
pub mod context;
pub mod debug_ui_renderer;
pub mod error;
pub mod game_renderer;
pub mod texture;
pub mod util;
pub mod vertex;

pub struct Renderer {
    pub render_context: RenderContext,

    pub game_renderer: GameRenderer,
    pub debug_ui_renderer: DebugUIRenderer,
}

impl Renderer {
    pub async fn new(window: Rc<Window>, camera: &Camera, debug_ui: &mut DebugUI) -> Self {
        let mut render_context = RenderContext::new(Rc::clone(&window)).await;
        let game_renderer = GameRenderer::new(&render_context, camera);
        let debug_ui_renderer = DebugUIRenderer::new(&render_context, debug_ui);

        Self {
            render_context,
            debug_ui_renderer,
            game_renderer,
        }
    }

    pub fn render(
        &mut self,
        camera: &Camera,
        debug_ui_render_state: &DebugUIRenderState,
        time_elapsed: &Duration,
    ) {
        self.render_context.render(
            &mut self.debug_ui_renderer,
            debug_ui_render_state,
            &self.game_renderer,
            camera,
        );
    }

    pub fn resize(&mut self, new_window_size: &PhysicalSize<u32>) {
        if new_window_size.width > 0 && new_window_size.height > 0 {
            self.render_context.resize(new_window_size);
        }
    }
}
