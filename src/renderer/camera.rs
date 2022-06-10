use crate::game;
use crate::game::camera::Camera;
use crate::renderer::context::RenderContext;
use crate::renderer::util::any_sized_as_u8_slice;
use nalgebra::{Matrix4, Perspective3};
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu::{BindingResource, Buffer};

pub struct CameraRenderer {
    buffer: Buffer,
}

impl CameraRenderer {
    pub fn new(render_context: &RenderContext, camera: &Camera) -> Self {
        let camera_buffer = CameraBuffer {
            projection: camera.get_projection_matrix(),
            view: camera.get_view_matrix(),
        };

        let buffer = render_context
            .device
            .create_buffer_init(&BufferInitDescriptor {
                label: Some("Create buffer: Buffer init descriptor"),
                contents: any_sized_as_u8_slice(&camera_buffer),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            });

        Self { buffer }
    }

    pub fn as_entire_binding(&self) -> BindingResource {
        self.buffer.as_entire_binding()
    }

    pub fn update(&self, render_context: &RenderContext, camera: &game::camera::Camera) {
        let camera_buffer = CameraBuffer {
            projection: camera.get_projection_matrix(),
            view: camera.get_view_matrix(),
        };

        render_context
            .queue
            .write_buffer(&self.buffer, 0, any_sized_as_u8_slice(&camera_buffer));
    }
}

pub struct CameraBuffer {
    projection: Matrix4<f32>,
    view: Matrix4<f32>,
}
