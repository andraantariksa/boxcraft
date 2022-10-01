use crate::game::transform::Transform;
use bitflags::bitflags;
use legion::system;
use nalgebra::{Matrix4, Vector, Vector2, Vector3};
use std::time::Duration;

bitflags! {
    struct Neighbor: u8 {
        const FRONT = 0b000001;
        const BACK = 0b000010;
        const RIGHT = 0b000100;
        const LEFT = 0b001000;
        const TOP = 0b010000;
        const BOTTOM = 0b100000;
        // const ABC = Self::A.bits | Self::B.bits | Self::C.bits;
    }
}

#[derive(Clone)]
pub struct Block {
    pub r#type: BlockType,
}

impl Block {
    pub const SIZE: f32 = 1.0;
    pub const HALF_SIZE: f32 = Self::SIZE / 2.0;

    pub fn new(r#type: BlockType) -> Self {
        Self { r#type }
    }
}

pub struct FacesRawInstance {
    model_transformation: Matrix4<f32>,
    texture_pos: Vector2<i32>,
}

impl FacesRawInstance {
    pub fn from(block: &Block, transform: &Transform) -> Self {
        // let mut faces = vec![];
        Self {
            model_transformation: transform.get_transformation_matrix(),
            texture_pos: Vector2::new(1, 0),
        }
    }

    pub fn vertex_buffer_layout<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<FacesRawInstance>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 10,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 11,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
                    shader_location: 12,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 12]>() as wgpu::BufferAddress,
                    shader_location: 13,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 16]>() as wgpu::BufferAddress,
                    shader_location: 14,
                    format: wgpu::VertexFormat::Sint32x2,
                },
            ],
        }
    }
}

#[derive(Clone)]
#[repr(u32)]
pub enum BlockType {
    Dirt,
    Grass,
    Cobblestone,
}

#[system(for_each)]
fn update_chunk(_block: &mut Block, #[resource] _time_elapsed: &Duration) {}
