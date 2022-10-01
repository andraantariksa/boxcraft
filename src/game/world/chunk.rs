use crate::game::transform::Transform;
use crate::game::world::block::{Block, BlockRawInstance};
use nalgebra::{Rotation3, Translation3, Vector3};

#[derive(Clone)]
pub struct Chunk {
    blocks: Vec<Vec<Vec<Option<Block>>>>,
}

impl Chunk {
    pub const CHUNK_SIDE_SIZE: usize = 3;
    pub const CHUNK_VERTICAL_SIZE: usize = 1; //256;

    pub fn from(block: Option<Block>) -> Self {
        Self {
            blocks: vec![
                vec![vec![block; Self::CHUNK_SIDE_SIZE]; Self::CHUNK_VERTICAL_SIZE];
                Self::CHUNK_SIDE_SIZE
            ],
        }
    }

    pub fn get_total_blocks() -> usize {
        Self::CHUNK_SIDE_SIZE * Self::CHUNK_SIDE_SIZE * Self::CHUNK_VERTICAL_SIZE
    }

    pub fn get_blocks(&self, center_point: &Vector3<f32>) -> Vec<BlockRawInstance> {
        let mut blocks = Vec::<BlockRawInstance>::with_capacity(Self::get_total_blocks());
        for (x, y_blocks) in self.blocks.iter().enumerate() {
            for (y, z_blocks) in y_blocks.iter().enumerate() {
                for (z, maybe_block) in z_blocks.iter().enumerate() {
                    if let Some(block) = maybe_block {
                        let w = -(Self::CHUNK_VERTICAL_SIZE as f32 * Block::SIZE / 2.0)
                            + Block::HALF_SIZE
                            + (Block::SIZE * y as f32);
                        let q = -(Self::CHUNK_SIDE_SIZE as f32 * Block::SIZE / 2.0)
                            + Block::HALF_SIZE
                            + (Block::SIZE * x as f32);
                        let e = -(Self::CHUNK_SIDE_SIZE as f32 * Block::SIZE / 2.0)
                            + Block::HALF_SIZE
                            + (Block::SIZE * z as f32);

                        blocks.push(BlockRawInstance::from(
                            block,
                            &Transform {
                                translation: Translation3::new(q, w, e),
                                rotation: Rotation3::identity(),
                            },
                        ));
                    }
                }
            }
        }
        blocks
    }
}
