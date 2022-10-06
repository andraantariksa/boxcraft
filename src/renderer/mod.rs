use crate::game::camera::Camera;
use crate::game::debug_ui::{DebugUI, DebugUIRenderState};
use crate::misc::window::Window;

use crate::renderer::context::RenderContext;
use crate::renderer::debug_ui_renderer::DebugUIRenderer;
use crate::renderer::game_renderer::GameRenderer;

use std::time::Duration;
use wgpu::{
    Color, LoadOp, Operations, RenderPassColorAttachment, RenderPassDepthStencilAttachment,
    RenderPassDescriptor,
};

use crate::game::world::World;
use winit::dpi::PhysicalSize;

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
    pub async fn new(window: &Window, camera: &Camera, debug_ui: &mut DebugUI) -> Self {
        let render_context = RenderContext::new(window).await;
        let game_renderer = GameRenderer::new(&render_context, window, camera);
        let debug_ui_renderer = DebugUIRenderer::new(&render_context, debug_ui);

        Self {
            debug_ui_renderer,
            game_renderer,
            render_context,
        }
    }

    pub fn render(
        &mut self,
        camera: &Camera,
        _time_elapsed: &Duration,
        window: &Window,
        debug_ui_render_state: &DebugUIRenderState,
        _world_blocks: &World,
    ) {
        self.game_renderer
            .prerender(&self.render_context, window, camera);

        let (mut command_encoder, texture, texture_view) =
            self.render_context.create_command_encoder();

        {
            let mut render_pass = command_encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("Render pass descriptor"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &texture_view,
                    resolve_target: None,
                    ops: Operations {
                        store: true,
                        load: LoadOp::Clear(Color {
                            r: 0.8,
                            g: 0.8,
                            b: 0.8,
                            a: 1.0,
                        }),
                    },
                })],
                depth_stencil_attachment: Some(RenderPassDepthStencilAttachment {
                    view: &self.game_renderer.depth_texture.texture_view,
                    depth_ops: Some(Operations {
                        load: LoadOp::Clear(1.0),
                        store: true,
                    }),
                    stencil_ops: None,
                }),
            });

            self.game_renderer.render(&mut render_pass);
        }

        {
            let mut render_pass = command_encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("Render pass descriptor"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &texture_view,
                    resolve_target: None,
                    ops: Operations {
                        store: true,
                        load: LoadOp::Load,
                    },
                })],
                depth_stencil_attachment: None,
            });
            self.debug_ui_renderer
                .render(
                    &self.render_context,
                    &mut render_pass,
                    debug_ui_render_state,
                )
                .unwrap();
        }

        self.render_context
            .queue
            .submit(core::iter::once(command_encoder.finish()));

        texture.present();
    }

    pub fn resize(&mut self, new_window_size: &PhysicalSize<u32>) {
        if new_window_size.width > 0 && new_window_size.height > 0 {
            self.render_context.resize(new_window_size);
        }
    }
}
