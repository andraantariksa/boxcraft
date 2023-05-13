pub mod block;
pub mod chunk;
pub mod generator;
pub mod plugin;
pub mod systems;
pub mod voronoi;
pub mod worker;

use crate::game::camera::Camera;
use bevy_ecs::prelude::*;
use std::borrow::Borrow;
use std::collections::{HashMap, HashSet};

use crate::boxworld::block::{Block, BlockType, RawFaceInstance};
use crate::boxworld::chunk::Chunk;

use crate::boxworld::worker::{BoxWorldTask, BoxWorldTaskResult};

use crate::renderer::game_renderer::GameRenderer;
use crate::renderer::Renderer;
use crate::worker::ComputeTaskPool;
use nalgebra::{try_convert, Vector2, Vector3};
use winit::dpi::Pixel;

#[derive(Resource)]
pub struct BoxWorld {
    visible_chunks: HashMap<Vector2<i32>, Chunk>,
    current_chunk_coord: Vector2<i32>,

    enqueued_chunk: HashSet<Vector2<i32>>,
    is_dirty: bool,
}

impl BoxWorld {
    pub const LEFT: Vector3<f32> = Vector3::new(-1.0, 0.0, 0.0);
    pub const RIGHT: Vector3<f32> = Vector3::new(1.0, 0.0, 0.0);
    pub const TOP: Vector3<f32> = Vector3::new(0.0, 1.0, 0.0);
    pub const BOTTOM: Vector3<f32> = Vector3::new(0.0, -1.0, 0.0);
    pub const FRONT: Vector3<f32> = Vector3::new(0.0, 0.0, 1.0);
    pub const BACK: Vector3<f32> = Vector3::new(0.0, 0.0, -1.0);

    pub const RENDER_CHUNK: usize = 3;
    pub const TOTAL_CHUNKS: usize = (Self::RENDER_CHUNK * 2 + 1) * (Self::RENDER_CHUNK * 2 + 1);
    pub const TOTAL_CHUNK_BLOCKS: usize = Chunk::MAXIMUM_TOTAL_BLOCKS * BoxWorld::TOTAL_CHUNKS;

    pub const WORKER_COUNT: usize = 2;

    pub fn new() -> Self {
        Self {
            visible_chunks: HashMap::with_capacity(Self::TOTAL_CHUNKS),
            current_chunk_coord: Vector2::new(i32::MAX, i32::MAX),
            is_dirty: true,
            enqueued_chunk: HashSet::new(),
        }
    }

    fn enqueue_work(&mut self, task_pool: &mut ComputeTaskPool, mut commands: Commands) {
        let needed_chunk_coord = self.needed_chunk_coord();

        self.enqueued_chunk.extend(&needed_chunk_coord);

        for chunk_coord in needed_chunk_coord {
            let task = task_pool.spawn(async move {
                let chunk = Chunk::with_block(Some(Block::new(BlockType::Dirt)), chunk_coord);
                BoxWorldTaskResult {
                    chunk,
                    coord: chunk_coord,
                }
            });
            commands.spawn(BoxWorldTask(task));
        }
    }

    pub fn update_current_chunk_coord(&mut self, camera: &Camera) -> bool {
        let current_chunk_coord =
            Self::get_chunk_coord_from_world_coord(&camera.position.xz().coords);
        let ret = self.current_chunk_coord != current_chunk_coord;
        self.current_chunk_coord = current_chunk_coord;
        ret
    }

    pub fn insert_chunk(&mut self, coord: Vector2<i32>, chunk: Chunk) {
        self.enqueued_chunk.remove(&coord);
        self.visible_chunks.insert(coord, chunk);
        self.is_dirty = true;
    }

    pub fn is_dirty(&self) -> bool {
        self.is_dirty
    }

    pub fn update_blocks(&mut self, renderer: &Renderer, game_renderer: &mut GameRenderer) {
        // Remove far blocks
        let max_diff = Self::RENDER_CHUNK as i32;

        let mut chunk_to_remove = Vec::new();
        for chunk_coord in self.visible_chunks.keys() {
            let diff: Vector2<i32> = chunk_coord - self.current_chunk_coord;
            let diagonal_diff = ((diff.component_mul(&diff)).sum() as f32).sqrt() as i32;
            if diagonal_diff > max_diff {
                chunk_to_remove.push(*chunk_coord);
            }
        }
        for chunk_coord in chunk_to_remove.iter() {
            self.visible_chunks.remove(chunk_coord);
        }

        let raw_face_instances = self.get_raw_face_instances();

        game_renderer.update_blocks(
            &renderer.render_context,
            &raw_face_instances
                .cloned()
                .collect::<Vec<RawFaceInstance>>(),
            self.get_raw_face_instances_len(),
        );

        self.is_dirty = false;
    }

    fn needed_chunk_coord(&self) -> HashSet<Vector2<i32>> {
        let corner_relative_coord = Self::RENDER_CHUNK as i32;

        let mut needed_chunk = HashSet::<Vector2<i32>>::new();
        for x in -corner_relative_coord..=corner_relative_coord {
            for z in -corner_relative_coord..=corner_relative_coord {
                let current_chunk_coord = self.current_chunk_coord + Vector2::new(x, z);
                needed_chunk.insert(current_chunk_coord);
            }
        }

        let visible_coords = self
            .visible_chunks
            .keys()
            .cloned()
            .collect::<HashSet<Vector2<i32>>>();
        (&needed_chunk - &visible_coords).borrow() - &self.enqueued_chunk
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

    pub fn get_raw_face_instances_len(&self) -> u32 {
        self.visible_chunks.values().fold(0usize, |len, chunk| {
            len + chunk.get_raw_face_instances().len()
        }) as u32
    }

    pub fn get_raw_face_instances(&self) -> impl Iterator<Item = &RawFaceInstance> {
        self.visible_chunks
            .values()
            .flat_map(|chunk| chunk.get_raw_face_instances())
    }
}

mod test {
    #[test]
    fn indices_to_world_coordinate() {}
}
