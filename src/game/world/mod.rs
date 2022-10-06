pub mod block;
pub mod chunk;
pub mod generator;

use crate::game::camera::Camera;

use crate::game::world::block::{Block, BlockType, FacesRawInstance};
use crate::game::world::chunk::Chunk;


use nalgebra::{try_convert, Vector2, Vector3};

pub struct World {
    visible_chunks: Vec<Vec<Chunk>>,
    center_point: Vector2<i32>,
}

impl World {
    pub const LEFT: Vector3<f32> = Vector3::new(-1.0, 0.0, 0.0);
    pub const RIGHT: Vector3<f32> = Vector3::new(1.0, 0.0, 0.0);
    pub const TOP: Vector3<f32> = Vector3::new(0.0, 1.0, 0.0);
    pub const BOTTOM: Vector3<f32> = Vector3::new(0.0, -1.0, 0.0);
    pub const FRONT: Vector3<f32> = Vector3::new(0.0, 0.0, 1.0);
    pub const BACK: Vector3<f32> = Vector3::new(0.0, 0.0, -1.0);

    pub const RENDER_CHUNK: usize = 2;

    pub fn from(camera: &Camera) -> Self {
        let visible_chunks = vec![
            vec![
                Chunk::from(Some(Block::new(BlockType::Cobblestone)));
                Self::RENDER_CHUNK * 2 + 1
            ];
            Self::RENDER_CHUNK * 2 + 1
        ];
        let center_point = try_convert::<Vector2<f32>, Vector2<i32>>(
            camera.position.coords.xz() + Vector2::from_element(Chunk::CHUNK_SIDE_SIZE / 2.0),
        )
        .unwrap()
            / Chunk::CHUNK_SIDE_SIZE as i32;
        Self {
            visible_chunks,
            center_point,
        }
    }

    pub fn refresh_visible_chunks(&mut self) {
        self.visible_chunks = vec![
            vec![
                Chunk::from(Some(Block::new(BlockType::Cobblestone)));
                Self::RENDER_CHUNK * 2 + 1
            ];
            Self::RENDER_CHUNK * 2 + 1
        ];
    }

    pub fn update(&mut self, camera: &Camera) -> bool {
        let new_center_point = try_convert::<Vector2<f32>, Vector2<i32>>(
            camera.position.coords.xz() + Vector2::from_element(Chunk::CHUNK_SIDE_SIZE / 2.0),
        )
        .unwrap()
            / Chunk::CHUNK_SIDE_SIZE as i32;
        if self.center_point != new_center_point {
            self.center_point = new_center_point;

            self.refresh_visible_chunks();
            return true;
        }
        false
    }

    pub fn get_total_chunks() -> usize {
        (Self::RENDER_CHUNK * 2) + 1
    }

    #[inline]
    fn get_world_center_block_coordinate() -> Vector2<usize> {
        Vector2::new(Self::RENDER_CHUNK, Self::RENDER_CHUNK)
    }

    pub fn get_block_raw_instances(&self) -> Vec<FacesRawInstance> {
        let maximum_total_blocks = Chunk::get_total_blocks() * Self::get_total_chunks();

        let mut block_raw_instances = Vec::with_capacity(maximum_total_blocks);
        for (x, z_chunks) in self.visible_chunks.iter().enumerate() {
            for (z, chunk) in z_chunks.iter().enumerate() {
                let block_coord = Vector2::new(x as i32, z as i32)
                    - Self::get_world_center_block_coordinate().cast::<i32>()
                    + self.center_point;
                let position_relative =
                    block_coord.cast::<f32>() * Block::SIZE * Chunk::CHUNK_SIDE_BLOCK as f32;
                let position_relative_3d =
                    Vector3::new(position_relative.x, 0.0, position_relative.y);
                block_raw_instances.extend(chunk.get_faces(&position_relative_3d).into_iter());
            }
        }
        block_raw_instances
    }
}

mod test {
    

    #[test]
    fn indices_to_world_coordinate() {}
}
