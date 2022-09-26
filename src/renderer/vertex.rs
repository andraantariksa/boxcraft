use nalgebra::{Point3, Vector3};
use std::mem;
use wgpu::{BufferAddress, VertexAttribute, VertexBufferLayout, VertexFormat, VertexStepMode};

pub struct Vertex {
    pub position: Point3<f32>,
    pub normal: Vector3<f32>,
}

pub trait VertexLike {
    fn vertex_buffer_layout<'a>() -> VertexBufferLayout<'a>;
}

impl VertexLike for Vertex {
    fn vertex_buffer_layout<'a>() -> VertexBufferLayout<'a> {
        VertexBufferLayout {
            array_stride: mem::size_of::<Vertex>() as BufferAddress,
            step_mode: VertexStepMode::Vertex,
            attributes: &[
                VertexAttribute {
                    format: VertexFormat::Float32x3,
                    offset: 0,
                    shader_location: 0,
                },
                VertexAttribute {
                    format: VertexFormat::Float32x3,
                    offset: mem::size_of::<Vector3<f32>>() as BufferAddress,
                    shader_location: 1,
                },
            ],
        }
    }
}
