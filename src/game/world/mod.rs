pub mod block;
pub mod chunk;
pub mod generator;

use crate::game::player::Player;
use crate::game::transform::Transform;
use crate::game::world::block::{Block, BlockType, FacesRawInstance};
use crate::game::world::chunk::Chunk;
use legion::{Entity, IntoQuery, World as ECSWorld};
use nalgebra::{Point2, Point3, Rotation3, Translation3, Vector3};

pub struct World {
    visible_chunks: Vec<Vec<Chunk>>,
    center_point: Vector3<f32>,
}

impl World {
    pub const LEFT: Vector3<f32> = Vector3::new(-1.0, 0.0, 0.0);
    pub const RIGHT: Vector3<f32> = Vector3::new(1.0, 0.0, 0.0);
    pub const TOP: Vector3<f32> = Vector3::new(0.0, 1.0, 0.0);
    pub const BOTTOM: Vector3<f32> = Vector3::new(0.0, -1.0, 0.0);
    pub const FRONT: Vector3<f32> = Vector3::new(0.0, 0.0, 1.0);
    pub const BACK: Vector3<f32> = Vector3::new(0.0, 0.0, -1.0);

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

    pub fn get_block_raw_instances(&self) -> Vec<FacesRawInstance> {
        let maximum_total_blocks = Chunk::get_total_blocks() * Self::get_total_chunks();

        let mut block_raw_instances = Vec::with_capacity(maximum_total_blocks);
        for z_chunks in self.visible_chunks.iter() {
            for chunk in z_chunks.iter() {
                let center_point = self.center_point;
                block_raw_instances.extend(chunk.get_faces(&center_point).into_iter());
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
