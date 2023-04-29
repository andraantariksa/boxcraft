use crate::game::camera::Camera;
use crate::misc::window::Window;
use crate::ui::{UIDrawData, UI};

use crate::renderer::context::RenderContext;
use crate::renderer::game_renderer::GameRenderer;

use bevy_ecs::prelude::*;
use std::time::Duration;
use wgpu::{
    Color, LoadOp, Operations, RenderPassColorAttachment, RenderPassDepthStencilAttachment,
    RenderPassDescriptor,
};

use crate::game::world::BoxWorld;
use crate::ui::renderer::DebugUIRenderer;
use winit::dpi::PhysicalSize;

pub mod camera;
pub mod context;
pub mod error;
pub mod game_renderer;
pub mod texture;
pub mod util;
pub mod vertex;

#[derive(Resource)]
pub struct Renderer {
    pub render_context: RenderContext,

    pub game_renderer: GameRenderer,
    pub ui_renderer: DebugUIRenderer,
}

impl Renderer {
    pub async fn new(window: &Window, camera: &Camera) -> Self {
        let render_context = RenderContext::new(window).await;
        let game_renderer = GameRenderer::new(&render_context, window, camera);
        let ui_renderer = DebugUIRenderer::new(&render_context);

        Self {
            ui_renderer,
            game_renderer,
            render_context,
        }
    }

    pub fn render(
        &mut self,
        camera: &Camera,
        _time_elapsed: &Duration,
        window: &Window,
        ui_render_state: UIDrawData,
        _world_blocks: &BoxWorld,
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
        self.ui_renderer.render(
            &mut command_encoder,
            &self.render_context,
            &texture_view,
            ui_render_state,
        );

        self.render_context
            .queue
            .submit(core::iter::once(command_encoder.finish()));

        self.ui_renderer.post_render();

        texture.present();
    }

    pub fn resize(&mut self, new_window_size: &PhysicalSize<u32>) {
        if new_window_size.width > 0 && new_window_size.height > 0 {
            self.render_context.resize(new_window_size);
            self.game_renderer.resize(&self.render_context);
        }
    }
}
