use crate::game::transform::Transform;
use bitflags::bitflags;
use legion::system;
use nalgebra::{Matrix4, Vector2};
use std::time::Duration;

bitflags! {
    #[derive(Clone, PartialEq)]
    pub struct BlockFace: u8 {
        const FRONT = 0b00000001;
        const BACK = 0b00000010;
        const RIGHT = 0b00000100;
        const LEFT = 0b00001000;
        const TOP = 0b00010000;
        const BOTTOM = 0b00100000;
    }
}

#[derive(Clone)]
pub struct Block {
    pub r#type: BlockType,
    pub face: BlockFace,
}

impl Block {
    pub const SIZE: f32 = 1.0;
    pub const HALF_SIZE: f32 = Self::SIZE / 2.0;

    pub const TOTAL_FACES: usize = 6;

    pub fn new(r#type: BlockType) -> Self {
        Self {
            r#type,
            face: BlockFace::empty(),
        }
    }

    pub fn get_texture_pos(r#type: BlockType, face: BlockFace) -> Vector2<i32> {
        match r#type {
            BlockType::Dirt => {
                if face == BlockFace::TOP {
                    Vector2::new(0, 0)
                } else if face == BlockFace::BOTTOM {
                    Vector2::new(2, 0)
                } else {
                    Vector2::new(1, 0)
                }
            }
            _ => unreachable!(), // BlockType::Grass => {}
                                 // BlockType::Cobblestone => {}
        }
    }
}

// #[warn(dead_code)]
#[derive(Clone)]
pub struct RawFaceInstance {
    model_transformation: Matrix4<f32>,
    texture_pos: Vector2<i32>,
}

impl RawFaceInstance {
    pub fn from(r#type: BlockType, face: BlockFace, transform: &Transform) -> Self {
        Self {
            model_transformation: transform.get_transformation_matrix(),
            texture_pos: Block::get_texture_pos(r#type, face),
        }
    }

    pub fn vertex_buffer_layout<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<RawFaceInstance>() as wgpu::BufferAddress,
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

#[derive(Clone, Copy)]
#[repr(u32)]
pub enum BlockType {
    Dirt,
    Grass,
    Cobblestone,
}

#[system(for_each)]
fn update_chunk(_block: &mut Block, #[resource] _time_elapsed: &Duration) {}
