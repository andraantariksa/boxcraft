pub mod block;
pub mod chunk;
pub mod generator;

use crate::game::camera::Camera;
use std::collections::HashMap;

use crate::game::world::block::{Block, BlockType, FacesRawInstance};
use crate::game::world::chunk::Chunk;

use nalgebra::{try_convert, Vector2, Vector3};

pub struct World {
    visible_chunks: Vec<Vec<Chunk>>,
    _visible_chunks: HashMap<Vector2<i32>, Chunk>,
    center_point_chunk_coord: Vector2<i32>,
    block_raw_instances: Vec<FacesRawInstance>,
}

impl World {
    pub const LEFT: Vector3<f32> = Vector3::new(-1.0, 0.0, 0.0);
    pub const RIGHT: Vector3<f32> = Vector3::new(1.0, 0.0, 0.0);
    pub const TOP: Vector3<f32> = Vector3::new(0.0, 1.0, 0.0);
    pub const BOTTOM: Vector3<f32> = Vector3::new(0.0, -1.0, 0.0);
    pub const FRONT: Vector3<f32> = Vector3::new(0.0, 0.0, 1.0);
    pub const BACK: Vector3<f32> = Vector3::new(0.0, 0.0, -1.0);

    pub const RENDER_CHUNK: usize = 2;
    pub const TOTAL_CHUNK: usize = (Self::RENDER_CHUNK * 2) + 1;

    pub fn from(camera: &Camera) -> Self {
        let visible_chunks = vec![
            vec![
                Chunk::from(Some(Block::new(BlockType::Cobblestone)));
                Self::RENDER_CHUNK * 2 + 1
            ];
            Self::RENDER_CHUNK * 2 + 1
        ];
        let center_point_chunk_coord =
            Self::chunk_coord_from_world_coord(&camera.position.xz().coords);

        let _visible_chunks = HashMap::with_capacity(World::TOTAL_CHUNK);
        for x in 1..center_point_chunk_coord {
            for x in 0..center_point_chunk_coord {}
        }

        const MAXIMUM_TOTAL_CHUNKS_BLOCKS: usize = Chunk::TOTAL_CHUNK * World::TOTAL_CHUNK;
        let block_raw_instances =
            Vec::with_capacity(MAXIMUM_TOTAL_CHUNKS_BLOCKS * Block::TOTAL_FACES);

        Self {
            visible_chunks,
            _visible_chunks,
            center_point_chunk_coord,
            block_raw_instances,
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
        let current_chunk_coord = Self::chunk_coord_from_world_coord(&camera.position.xz().coords);
        if self.center_point_chunk_coord == current_chunk_coord {
            return false;
        }

        self.center_point_chunk_coord = current_chunk_coord;
        self.refresh_visible_chunks();

        true
    }

    #[inline]
    fn get_world_center_block_coordinate() -> Vector2<usize> {
        Vector2::new(Self::RENDER_CHUNK, Self::RENDER_CHUNK)
    }

    pub fn get_chunk_coord_from_world_coord(world_coord: &Vector2<f32>) -> Vector2<i32> {
        unsafe { try_convert(world_coord / Chunk::CHUNK_SIDE_SIZE).unwrap_unchecked() }
    }

    pub fn get_world_coord_from_chunk_coord(world_coord: &Vector2<i32>) -> Vector2<f32> {
        world_coord * Chunk::CHUNK_SIDE_SIZE + Chunk::CHUNK_HALF_SIDE_SIZE
    }

    pub fn _get_block_raw_instances(&mut self) -> &Vec<FacesRawInstance> {
        self.block_raw_instances.clear();

        for (coord, chunk) in self._visible_chunks.iter() {
            let center_point = Self::get_world_coord_from_chunk_coord(coord);
            chunk.get_faces(
                &mut self.block_raw_instances,
                &Vector3::new(center_point.x, center_point.y, center_point.z),
            );
        }

        &self.block_raw_instances
    }

    pub fn get_block_raw_instances(&mut self) -> &Vec<FacesRawInstance> {
        self.block_raw_instances.clear();

        for (x, z_chunks) in self.visible_chunks.iter().enumerate() {
            for (z, chunk) in z_chunks.iter().enumerate() {
                let block_coord = Vector2::new(x as i32, z as i32)
                    - Self::get_world_center_block_coordinate().cast::<i32>()
                    + self.center_point_chunk_coord;
                let position_relative =
                    block_coord.cast::<f32>() * Block::SIZE * Chunk::CHUNK_SIDE_BLOCK as f32;
                let position_relative_3d =
                    Vector3::new(position_relative.x, 0.0, position_relative.y);
                chunk.get_faces(&mut self.block_raw_instances, &position_relative_3d);
            }
        }

        &self.block_raw_instances
    }
}

mod test {

    #[test]
    fn indices_to_world_coordinate() {}
}
