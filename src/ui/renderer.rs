use crate::ui::{UIDrawData};
use crate::renderer::context::RenderContext;
use egui::TexturesDelta;

use egui_wgpu_backend::{RenderPass as EguiRenderPass, ScreenDescriptor};

use wgpu::{CommandEncoder, TextureView};

pub struct DebugUIRenderer {
    egui_rpass: EguiRenderPass,
    texture_delta: Option<TexturesDelta>,
}

impl DebugUIRenderer {
    pub fn new(render_context: &RenderContext) -> Self {
        let egui_rpass = EguiRenderPass::new(
            &render_context.device,
            render_context.render_surface_config.format,
            1,
        );
        Self {
            egui_rpass,
            texture_delta: None,
        }
    }

    pub fn render(
        &mut self,
        encoder: &mut CommandEncoder,
        render_context: &RenderContext,
        output_view: &TextureView,
        draw_data: UIDrawData,
    ) {
        self.egui_rpass
            .add_textures(
                &render_context.device,
                &render_context.queue,
                &draw_data.textures_delta,
            )
            .unwrap();
        let screen_descriptor = ScreenDescriptor {
            scale_factor: render_context.scale_factor as f32,
            physical_height: render_context.render_surface_config.height,
            physical_width: render_context.render_surface_config.width,
        };
        self.egui_rpass.update_buffers(
            &render_context.device,
            &render_context.queue,
            &draw_data.paint_jobs,
            &screen_descriptor,
        );
        self.egui_rpass
            .execute(
                encoder,
                output_view,
                &draw_data.paint_jobs,
                &screen_descriptor,
                None,
            )
            .unwrap();
        self.texture_delta = Some(draw_data.textures_delta);
    }

    pub fn post_render(&mut self) {
        self.egui_rpass
            .remove_textures(self.texture_delta.take().unwrap())
            .unwrap();
    }
}
