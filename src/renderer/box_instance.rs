use crate::game::transform::Transform;
use crate::game::world::block::Block;
use crate::renderer::vertex::VertexLike;
use nalgebra::{Matrix4, Point2, Vector4};
use std::mem;
use wgpu::{BufferAddress, VertexAttribute, VertexBufferLayout, VertexFormat, VertexStepMode};

pub struct BoxInstance {
    pub transformation: Matrix4<f32>,
    pub texture_map: TextureCoord,
}

impl VertexLike for BoxInstance {
    fn vertex_buffer_layout<'a>() -> VertexBufferLayout<'a> {
        VertexBufferLayout {
            array_stride: mem::size_of::<BoxInstance>() as BufferAddress,
            step_mode: VertexStepMode::Instance,
            attributes: &[
                // Transformation
                VertexAttribute {
                    format: VertexFormat::Float32x4,
                    offset: 0,
                    shader_location: 2,
                },
                VertexAttribute {
                    format: VertexFormat::Float32x4,
                    offset: mem::size_of::<Vector4<f32>>() as BufferAddress,
                    shader_location: 3,
                },
                VertexAttribute {
                    format: VertexFormat::Float32x4,
                    offset: (mem::size_of::<Vector4<f32>>() * 2) as BufferAddress,
                    shader_location: 4,
                },
                VertexAttribute {
                    format: VertexFormat::Float32x4,
                    offset: (mem::size_of::<Vector4<f32>>() * 3) as BufferAddress,
                    shader_location: 5,
                },
                // Texture coord
                VertexAttribute {
                    format: VertexFormat::Uint8x2,
                    offset: (mem::size_of::<Matrix4<f32>>() * 4) as BufferAddress,
                    shader_location: 6,
                },
            ],
        }
    }
}

pub type TextureCoord = Point2<u8>;

fn to_box_transform(transform: &Transform, block: &Block) -> BoxInstance {
    BoxInstance {
        transformation: transform.translation.to_homogeneous()
            * transform.rotation.to_homogeneous(),
        texture_map: Point2::new(0, 0),
    }
}
