use nalgebra::{Matrix4, Vector4};
use std::mem;
use wgpu::{BufferAddress, VertexAttribute, VertexBufferLayout, VertexFormat, VertexStepMode};

pub struct BoxInstance {
    pub transformation: Matrix4<f32>,
    pub texture_map: TextureMap,
}

impl BoxInstance {
    pub fn buffer_layout<'a>() -> VertexBufferLayout<'a> {
        VertexBufferLayout {
            array_stride: mem::size_of::<BoxInstance>() as BufferAddress,
            step_mode: VertexStepMode::Instance,
            attributes: &[
                VertexAttribute {
                    format: VertexFormat::Float32x4,
                    offset: mem::size_of::<Vector4<f32>>() as BufferAddress,
                    shader_location: 1,
                },
                VertexAttribute {
                    format: VertexFormat::Float32x4,
                    offset: mem::size_of::<Vector4<f32>>() as BufferAddress,
                    shader_location: 2,
                },
                VertexAttribute {
                    format: VertexFormat::Float32x4,
                    offset: mem::size_of::<Vector4<f32>>() as BufferAddress,
                    shader_location: 3,
                },
                VertexAttribute {
                    format: VertexFormat::Float32x4,
                    offset: mem::size_of::<Vector4<f32>>() as BufferAddress,
                    shader_location: 4,
                },
                VertexAttribute {
                    format: VertexFormat::Uint8x2,
                    offset: mem::size_of::<Matrix4<f32>>() as BufferAddress,
                    shader_location: 5,
                },
            ],
        }
    }
}

pub type TextureMap = (u8, u8);
