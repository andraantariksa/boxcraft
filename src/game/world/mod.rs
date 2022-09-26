pub mod generator;

use crate::game::block::{Block, BlockRawInstance, BlockType};
use crate::game::player::Player;
use crate::game::transform::Transform;
use legion::{Entity, IntoQuery, World as ECSWorld};
use nalgebra::{Point2, Point3, Rotation3, Translation3, Vector3};

#[derive(Clone)]
pub struct Chunk {
    blocks: Vec<Vec<Vec<Option<Block>>>>,
}

impl Chunk {
    pub const CHUNK_SIDE_SIZE: usize = 16;
    pub const CHUNK_VERTICAL_SIZE: usize = 3; //256;

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

pub struct WorldBlocks {
    visible_chunks: Vec<Vec<Chunk>>,
    center_point: Vector3<f32>,
}

impl WorldBlocks {
    pub const RENDER_CHUNK: usize = 0;

    pub fn from(player: &Player) -> Self {
        let mut visible_chunks =
            vec![vec![Chunk::from(None); Self::RENDER_CHUNK * 2 + 1]; Self::RENDER_CHUNK * 2 + 1];
        for z_chunks in visible_chunks.iter_mut() {
            for chunk in z_chunks.iter_mut() {
                *chunk = Chunk::from(Some(Block::new(BlockType::Cobblestone)));
            }
        }
        Self {
            visible_chunks,
            center_point: Vector3::new(0.0, 0.0, 0.0),
        }
    }

    pub fn update_user_position(&self, player: &Player) {}

    pub fn get_total_chunks() -> usize {
        (Self::RENDER_CHUNK * 2) + 1
    }

    pub fn get_block_raw_instances(&self) -> Vec<BlockRawInstance> {
        let maximum_total_blocks = Chunk::get_total_blocks() * Self::get_total_chunks();

        let mut block_raw_instances = Vec::with_capacity(maximum_total_blocks);
        for (x, z_chunks) in self.visible_chunks.iter().enumerate() {
            for (z, chunk) in z_chunks.iter().enumerate() {
                let center_point = self.center_point;
                block_raw_instances.extend(chunk.get_blocks(&center_point).into_iter());
            }
        }
        block_raw_instances
    }
}

mod test {
    use super::*;

    #[test]
    fn indices_to_world_coordinate() {}
}
