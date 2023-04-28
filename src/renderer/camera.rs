use crate::game::camera::Camera;
use crate::renderer::context::RenderContext;
use crate::renderer::util::any_sized_as_u8_slice;
use nalgebra::{Matrix4, Point3};
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu::{BindingResource, Buffer};
use winit::window::Window;

pub struct CameraRenderer {
    buffer: Buffer,
}

impl CameraRenderer {
    pub fn new(render_context: &RenderContext, window: &Window, camera: &Camera) -> Self {
        let window_size = window.inner_size();
        let aspect_ratio = window_size.width as f32 / window_size.height as f32;

        let buffer = render_context
            .device
            .create_buffer_init(&BufferInitDescriptor {
                label: Some("Create buffer: Buffer init descriptor"),
                contents: any_sized_as_u8_slice(&camera.get_raw_buffer(aspect_ratio)),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            });

        Self { buffer }
    }

    pub fn as_entire_binding(&self) -> BindingResource {
        self.buffer.as_entire_binding()
    }

    pub fn update(&self, render_context: &RenderContext, window: &Window, camera: &Camera) {
        let window_size = window.inner_size();
        let aspect_ratio = window_size.width as f32 / window_size.height as f32;

        render_context.queue.write_buffer(
            &self.buffer,
            0,
            any_sized_as_u8_slice(&camera.get_raw_buffer(aspect_ratio)),
        );
    }
}

pub struct CameraBuffer {
    pub projection: Matrix4<f32>,
    pub view: Matrix4<f32>,
    pub position: Point3<f32>,
    pub _p0: f32,
}
