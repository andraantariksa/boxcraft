pub mod block;
pub mod chunk;
pub mod generator;

use crate::game::camera::Camera;
use std::collections::{HashMap, VecDeque};

use crate::game::world::block::{Block, BlockType, RawFaceInstance};
use crate::game::world::chunk::Chunk;

use nalgebra::{try_convert, Point2, Vector2, Vector3};
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use tokio::task::JoinHandle;
use wgpu::PolygonMode::Point;

pub struct World {
    visible_chunks: VecDeque<VecDeque<Chunk>>,
    center_point_chunk_coord: Vector2<i32>,
    raw_face_instances: Vec<RawFaceInstance>,
    sender: UnboundedSender<Vec<RawFaceInstance>>,
    pub receiver: UnboundedReceiver<Vec<RawFaceInstance>>,
    handler: Option<JoinHandle<()>>,
}

impl World {
    pub const LEFT: Vector3<f32> = Vector3::new(-1.0, 0.0, 0.0);
    pub const RIGHT: Vector3<f32> = Vector3::new(1.0, 0.0, 0.0);
    pub const TOP: Vector3<f32> = Vector3::new(0.0, 1.0, 0.0);
    pub const BOTTOM: Vector3<f32> = Vector3::new(0.0, -1.0, 0.0);
    pub const FRONT: Vector3<f32> = Vector3::new(0.0, 0.0, 1.0);
    pub const BACK: Vector3<f32> = Vector3::new(0.0, 0.0, -1.0);

    pub const RENDER_CHUNK: usize = 2;
    pub const TOTAL_CHUNKS: usize = (Self::RENDER_CHUNK * 2 + 1) * 2;
    pub const TOTAL_CHUNK_BLOCKS: usize = Chunk::TOTAL_BLOCKS * World::TOTAL_CHUNKS;

    pub fn from(camera: &Camera) -> Self {
        let center_point_chunk_coord =
            Self::get_chunk_coord_from_world_coord(&camera.position.xz().coords);
        let corner_relative_coord = Self::RENDER_CHUNK as i32;
        let mut raw_face_instances =
            Vec::with_capacity(Self::TOTAL_CHUNK_BLOCKS * Block::TOTAL_FACES);

        let mut visible_chunks = VecDeque::with_capacity(Self::RENDER_CHUNK * 2 + 1);
        for x in -corner_relative_coord..=corner_relative_coord {
            let mut x_chunks = VecDeque::with_capacity(Self::RENDER_CHUNK * 2 + 1);
            for z in -corner_relative_coord..=corner_relative_coord {
                let current_chunk_coord = center_point_chunk_coord + Vector2::new(x, z);
                let chunk_center_point =
                    Self::get_world_coord_from_chunk_coord(&current_chunk_coord);

                let chunk = Chunk::from(Some(Block::new(BlockType::Dirt)));
                chunk.get_raw_face_instances(
                    &mut raw_face_instances,
                    &Vector3::new(chunk_center_point.x, 0.0, chunk_center_point.y),
                );
                x_chunks.push_back(chunk);
            }

            visible_chunks.push_back(x_chunks);
        }

        let (sender, receiver) = unbounded_channel();

        Self {
            visible_chunks,
            center_point_chunk_coord,
            raw_face_instances,
            receiver,
            sender,
            handler: None,
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
                let chunk_center_point =
                    Self::get_world_coord_from_chunk_coord(&current_chunk_coord);

                let chunk = Chunk::from(Some(Block::new(BlockType::Dirt)));
                chunk.get_raw_face_instances(
                    &mut block_raw_instances,
                    &Vector3::new(chunk_center_point.x, 0.0, chunk_center_point.y),
                );
            }
        }

        block_raw_instances
    }

    pub fn update(&mut self, camera: &Camera) {
        let current_chunk_coord =
            Self::get_chunk_coord_from_world_coord(&camera.position.xz().coords);
        if self.center_point_chunk_coord == current_chunk_coord {
            return;
        }

        self.handler = Some(tokio::spawn(async {
            let a = self.get_faces();
            self.sender.send(a);
        }));
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
