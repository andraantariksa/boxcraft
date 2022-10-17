pub mod block;
pub mod chunk;
pub mod generator;

use crate::game::camera::Camera;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet, VecDeque};

use crate::game::world::block::{Block, BlockType, RawFaceInstance};
use crate::game::world::chunk::Chunk;

use nalgebra::{try_convert, Point2, Vector2, Vector3};
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};

pub struct World {
    visible_chunks: HashMap<Vector2<i32>, Chunk>,
    center_point_chunk_coord: Vector2<i32>,
    raw_face_instances: Vec<RawFaceInstance>,
}

impl World {
    pub const LEFT: Vector3<f32> = Vector3::new(-1.0, 0.0, 0.0);
    pub const RIGHT: Vector3<f32> = Vector3::new(1.0, 0.0, 0.0);
    pub const TOP: Vector3<f32> = Vector3::new(0.0, 1.0, 0.0);
    pub const BOTTOM: Vector3<f32> = Vector3::new(0.0, -1.0, 0.0);
    pub const FRONT: Vector3<f32> = Vector3::new(0.0, 0.0, 1.0);
    pub const BACK: Vector3<f32> = Vector3::new(0.0, 0.0, -1.0);

    pub const RENDER_CHUNK: usize = 1;
    pub const TOTAL_CHUNKS: usize = (Self::RENDER_CHUNK * 2 + 1) * (Self::RENDER_CHUNK * 2 + 1);
    pub const TOTAL_CHUNK_BLOCKS: usize = Chunk::MAXIMUM_TOTAL_BLOCKS * World::TOTAL_CHUNKS;

    pub fn from(camera: &Camera) -> Self {
        let corner_relative_coord = Self::RENDER_CHUNK as i32;

        let center_point_chunk_coord =
            Self::get_chunk_coord_from_world_coord(&camera.position.xz().coords);
        let mut raw_face_instances =
            Vec::with_capacity(Self::TOTAL_CHUNK_BLOCKS * Block::TOTAL_FACES);
        let mut visible_chunks = HashMap::with_capacity(Self::TOTAL_CHUNKS);

        for x in -corner_relative_coord..=corner_relative_coord {
            for z in -corner_relative_coord..=corner_relative_coord {
                let chunk_coord = center_point_chunk_coord + Vector2::new(x, z);

                let chunk = Chunk::with_block(Some(Block::new(BlockType::Dirt)), chunk_coord);
                raw_face_instances.extend(chunk.get_raw_face_instances().into_iter());
                visible_chunks.insert(chunk_coord, chunk);
            }
        }

        Self {
            visible_chunks,
            center_point_chunk_coord,
            raw_face_instances,
        }
    }

    pub fn get_faces(&self) -> Vec<RawFaceInstance> {
        let center_point_chunk_coord = self.center_point_chunk_coord;
        let corner_relative_coord = Self::RENDER_CHUNK as i32;
        let mut block_raw_instances =
            Vec::with_capacity(Self::TOTAL_CHUNK_BLOCKS * Block::TOTAL_FACES);

        for x in -corner_relative_coord..=corner_relative_coord {
            for z in -corner_relative_coord..=corner_relative_coord {
                let current_chunk_coord = center_point_chunk_coord + Vector2::new(x, z);

                let chunk =
                    Chunk::with_block(Some(Block::new(BlockType::Dirt)), current_chunk_coord);
                let raw_face_instances = chunk.get_raw_face_instances();
                block_raw_instances.extend(raw_face_instances.into_iter());
            }
        }

        block_raw_instances
    }

    pub fn update(&mut self, camera: &Camera) -> bool {
        let current_chunk_coord =
            Self::get_chunk_coord_from_world_coord(&camera.position.xz().coords);

        let is_moved_chunk = self.center_point_chunk_coord != current_chunk_coord;
        if is_moved_chunk {
            self.center_point_chunk_coord = current_chunk_coord;

            let corner_relative_coord = Self::RENDER_CHUNK as i32;

            self.raw_face_instances.clear();

            let mut needed_chunk = HashSet::<Vector2<i32>>::new();
            for x in -corner_relative_coord..=corner_relative_coord {
                for z in -corner_relative_coord..=corner_relative_coord {
                    let current_chunk_coord = self.center_point_chunk_coord + Vector2::new(x, z);
                    needed_chunk.insert(current_chunk_coord);
                }
            }

            let mut chunk_to_remove = Vec::new();
            for chunk_coord in self.visible_chunks.keys() {
                needed_chunk.remove(chunk_coord);

                let diff = chunk_coord - current_chunk_coord;
                let diagonal_diff =
                    (diff.x as f32 * diff.x as f32 + diff.y as f32 * diff.y as f32).sqrt() as i32;
                let max_diff = Self::RENDER_CHUNK as i32;
                if diagonal_diff > max_diff {
                    chunk_to_remove.push(chunk_coord.clone());
                }
            }
            for chunk_coord in chunk_to_remove.iter() {
                self.visible_chunks.remove(chunk_coord);
            }

            let chunks = needed_chunk
                .into_par_iter()
                .map(|chunk_coord| {
                    (
                        chunk_coord,
                        Chunk::with_block(Some(Block::new(BlockType::Dirt)), chunk_coord),
                    )
                })
                .collect::<Vec<(Vector2<i32>, Chunk)>>();
            for (chunk_coord, chunk) in chunks {
                self.visible_chunks.insert(chunk_coord, chunk);
            }

            for chunk in self.visible_chunks.values() {
                self.raw_face_instances
                    .extend(chunk.get_raw_face_instances())
            }
        }

        is_moved_chunk
    }

    #[inline]
    fn get_world_center_block_coordinate() -> Vector2<usize> {
        Vector2::new(Self::RENDER_CHUNK, Self::RENDER_CHUNK)
    }

    pub fn get_chunk_coord_from_world_coord(world_coord: &Vector2<f32>) -> Vector2<i32> {
        unsafe { try_convert(world_coord / Chunk::CHUNK_SIDE_SIZE).unwrap_unchecked() }
    }

    pub fn get_world_coord_from_chunk_coord(world_coord: &Vector2<i32>) -> Vector2<f32> {
        (world_coord * Chunk::CHUNK_SIDE_SIZE as i32
            + Vector2::from_element(Chunk::CHUNK_HALF_SIDE_SIZE as i32))
        .cast::<f32>()
    }

    pub fn get_block_raw_instances(&mut self) -> &Vec<RawFaceInstance> {
        &self.raw_face_instances
    }
}

mod test {

    #[test]
    fn indices_to_world_coordinate() {}
}
