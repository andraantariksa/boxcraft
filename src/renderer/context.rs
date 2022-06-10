use crate::game::camera::Camera;
use crate::game::debug_ui::DebugUIRenderState;
use crate::renderer::debug_ui_renderer::DebugUIRenderer;
use crate::renderer::game_renderer::GameRenderer;
use std::rc::Rc;
use wgpu::{
    Color, CommandEncoderDescriptor, LoadOp, Operations, PresentMode, RenderPass,
    RenderPassColorAttachment, RenderPassDescriptor, TextureFormat, TextureViewDescriptor,
};
use winit::dpi::PhysicalSize;
use winit::window::Window;

pub struct RenderContext {
    pub render_surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub render_surface_config: wgpu::SurfaceConfiguration,
    pub window: Rc<Window>,
}

impl RenderContext {
    pub async fn new(window: Rc<Window>) -> Self {
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let render_surface = unsafe { instance.create_surface(&*window) };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&render_surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                    label: None,
                },
                None,
            )
            .await
            .unwrap();

        let window_size = window.inner_size();
        let render_surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: TextureFormat::Bgra8UnormSrgb,
            width: window_size.width,
            height: window_size.height,
            present_mode: PresentMode::Immediate,
        };
        render_surface.configure(&device, &render_surface_config);

        Self {
            render_surface,
            render_surface_config,
            queue,
            device,
            window,
        }
    }

    pub fn resize(&mut self, new_window_size: &PhysicalSize<u32>) {
        self.render_surface_config.width = new_window_size.width;
        self.render_surface_config.height = new_window_size.height;

        self.render_surface
            .configure(&self.device, &self.render_surface_config);
    }

    pub fn render(
        &self,
        debug_ui_renderer: &mut DebugUIRenderer,
        debug_ui_render_state: &DebugUIRenderState,
        game_renderer: &GameRenderer,
        camera: &Camera,
    )
    // pub fn render<F>(&self, render_pass_recording: F)
    // where
    //     F: FnOnce(&mut RenderPass),
    {
        game_renderer.prerender(&self, camera);

        let texture_to_present = self.render_surface.get_current_texture().unwrap();
        let texture_view_to_present = texture_to_present
            .texture
            .create_view(&TextureViewDescriptor::default());

        let mut command_encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("Command encoder descriptor"),
            });

        {
            let mut render_pass = command_encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("Render pass descriptor"),
                color_attachments: &[RenderPassColorAttachment {
                    view: &texture_view_to_present,
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
                }],
                depth_stencil_attachment: None,
            });

            game_renderer.render(&mut render_pass);

            debug_ui_renderer
                .renderer
                .render(
                    debug_ui_render_state.draw_data,
                    &self.queue,
                    &self.device,
                    &mut render_pass,
                )
                .unwrap();
        }

        self.queue
            .submit(core::iter::once(command_encoder.finish()));

        texture_to_present.present();
    }
}
