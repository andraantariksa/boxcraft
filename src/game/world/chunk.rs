use crate::game::transform::Transform;
use crate::game::world::block::{Block, RawFaceInstance};
use crate::game::world::World;
use nalgebra::{Rotation3, Translation3, Vector2, Vector3};

use super::block::BlockFace;

#[derive(Clone)]
pub struct Chunk {
    blocks: Vec<Vec<Vec<Option<Block>>>>,
    chunk_coord: Vector2<i32>,
    world_coord: Vector3<f32>,
    raw_face_instances: Vec<RawFaceInstance>,
}

impl Chunk {
    pub const CHUNK_SIDE_BLOCK: usize = 16;
    pub const CHUNK_VERTICAL_BLOCK: usize = 2;
    pub const MAXIMUM_TOTAL_BLOCKS: usize =
        Self::CHUNK_SIDE_BLOCK * Self::CHUNK_SIDE_BLOCK * Self::CHUNK_VERTICAL_BLOCK;

    pub const CHUNK_SIDE_SIZE: f32 = Self::CHUNK_SIDE_BLOCK as f32 * Block::SIZE;
    pub const CHUNK_HALF_SIDE_SIZE: f32 = Self::CHUNK_SIDE_SIZE as f32 * 0.5;

    pub fn with_block(block: Option<Block>, chunk_coord: Vector2<i32>) -> Self {
        let mut blocks = vec![
            vec![vec![block; Self::CHUNK_SIDE_BLOCK]; Self::CHUNK_VERTICAL_BLOCK];
            Self::CHUNK_SIDE_BLOCK
        ];
        Self::with_blocks(blocks, chunk_coord)
    }

    pub fn with_blocks(blocks: Vec<Vec<Vec<Option<Block>>>>, chunk_coord: Vector2<i32>) -> Self {
        let world_coord_xz = World::get_world_coord_from_chunk_coord(&chunk_coord);
        let mut instance = Self {
            blocks,
            raw_face_instances: Vec::with_capacity(Self::MAXIMUM_TOTAL_BLOCKS),
            world_coord: Vector3::new(world_coord_xz.x, 0.0, world_coord_xz.y),
            chunk_coord,
        };
        instance.calculate_faces();
        instance.calculate_raw_face_instances();
        instance
    }

    fn calculate_faces(&mut self) {
        for x in 0..Self::CHUNK_SIDE_BLOCK {
            for y in 0..Self::CHUNK_VERTICAL_BLOCK {
                for z in 0..Self::CHUNK_SIDE_BLOCK {
                    let mut face = BlockFace::empty();
                    {
                        let y_blocks = unsafe { self.blocks.get_unchecked(x) };
                        let z_blocks = unsafe { y_blocks.get_unchecked(y) };
                        let maybe_block = unsafe { z_blocks.get_unchecked(z) };

                        if maybe_block.is_some() {
                            // X
                            {
                                let _x = x + 1;
                                if self.blocks.len() >= _x
                                    || unsafe {
                                        self.blocks
                                            .get_unchecked(_x)
                                            .get_unchecked(y)
                                            .get_unchecked(z)
                                    }
                                    .is_none()
                                {
                                    face |= BlockFace::RIGHT;
                                }
                            }
                            {
                                if x == 0
                                    || unsafe {
                                        self.blocks
                                            .get_unchecked(x - 1)
                                            .get_unchecked(y)
                                            .get_unchecked(z)
                                    }
                                    .is_none()
                                {
                                    face |= BlockFace::LEFT;
                                }
                            }

                            // Y
                            {
                                let _y = y + 1;
                                if y_blocks.len() >= _y
                                    || unsafe { y_blocks.get_unchecked(_y).get_unchecked(z) }
                                        .is_none()
                                {
                                    face |= BlockFace::TOP;
                                }
                            }
                            {
                                if y == 0
                                    || unsafe { y_blocks.get_unchecked(y - 1).get_unchecked(z) }
                                        .is_none()
                                {
                                    face |= BlockFace::BOTTOM;
                                }
                            }

                            // Z
                            {
                                if z_blocks.len() >= z + 1 {
                                    face |= BlockFace::FRONT;
                                }
                            }
                            {
                                if z == 0 || unsafe { z_blocks.get_unchecked(z - 1) }.is_none() {
                                    face |= BlockFace::BACK;
                                }
                            }
                        }
                    }

                    let y_blocks = unsafe { self.blocks.get_unchecked_mut(x) };
                    let z_blocks = unsafe { y_blocks.get_unchecked_mut(y) };
                    let maybe_block = unsafe { z_blocks.get_unchecked_mut(z) };

                    if !face.is_empty() {
                        unsafe { maybe_block.as_mut().unwrap_unchecked() }.face |= face;
                    }
                }
            }
        }
    }

    pub fn calculate_raw_face_instances(&mut self) {
        self.raw_face_instances.clear();

        for x in 0..Self::CHUNK_SIDE_BLOCK {
            for y in 0..Self::CHUNK_VERTICAL_BLOCK {
                for z in 0..Self::CHUNK_SIDE_BLOCK {
                    let y_blocks = unsafe { self.blocks.get_unchecked(x) };
                    let z_blocks = unsafe { y_blocks.get_unchecked(y) };
                    let maybe_block = unsafe { z_blocks.get_unchecked(z) };

                    if let Some(block) = maybe_block {
                        let yy = -(Self::CHUNK_VERTICAL_BLOCK as f32 * Block::SIZE / 2.0)
                            + Block::HALF_SIZE
                            + (Block::SIZE * y as f32);
                        let xx = -(Self::CHUNK_SIDE_BLOCK as f32 * Block::SIZE / 2.0)
                            + Block::HALF_SIZE
                            + (Block::SIZE * x as f32);
                        let zz = -(Self::CHUNK_SIDE_BLOCK as f32 * Block::SIZE / 2.0)
                            + Block::HALF_SIZE
                            + (Block::SIZE * z as f32);

                        if block.face.contains(BlockFace::RIGHT) {
                            self.raw_face_instances.push(RawFaceInstance::from(
                                block,
                                &Transform {
                                    translation: Translation3::from(
                                        Vector3::new(xx + Block::HALF_SIZE, yy, zz)
                                            + self.world_coord,
                                    ),
                                    rotation: Rotation3::new(Vector3::new(
                                        0.0,
                                        std::f32::consts::FRAC_PI_2,
                                        0.0,
                                    )),
                                },
                            ));
                        }
                        if block.face.contains(BlockFace::LEFT) {
                            self.raw_face_instances.push(RawFaceInstance::from(
                                block,
                                &Transform {
                                    translation: Translation3::from(
                                        Vector3::new(xx - Block::HALF_SIZE, yy, zz)
                                            + self.world_coord,
                                    ),
                                    rotation: Rotation3::new(Vector3::new(
                                        0.0,
                                        -std::f32::consts::FRAC_PI_2,
                                        0.0,
                                    )),
                                },
                            ));
                        }
                        if block.face.contains(BlockFace::TOP) {
                            self.raw_face_instances.push(RawFaceInstance::from(
                                block,
                                &Transform {
                                    translation: Translation3::from(
                                        Vector3::new(xx, yy + Block::HALF_SIZE, zz)
                                            + self.world_coord,
                                    ),
                                    rotation: Rotation3::new(Vector3::new(
                                        -std::f32::consts::FRAC_PI_2,
                                        0.0,
                                        0.0,
                                    )),
                                },
                            ));
                        }
                        if block.face.contains(BlockFace::BOTTOM) {
                            self.raw_face_instances.push(RawFaceInstance::from(
                                block,
                                &Transform {
                                    translation: Translation3::from(
                                        Vector3::new(xx, yy - Block::HALF_SIZE, zz)
                                            + self.world_coord,
                                    ),
                                    rotation: Rotation3::new(Vector3::new(
                                        std::f32::consts::FRAC_PI_2,
                                        0.0,
                                        0.0,
                                    )),
                                },
                            ));
                        }
                        if block.face.contains(BlockFace::FRONT) {
                            self.raw_face_instances.push(RawFaceInstance::from(
                                block,
                                &Transform {
                                    translation: Translation3::from(
                                        Vector3::new(xx, yy, zz + Block::HALF_SIZE)
                                            + self.world_coord,
                                    ),
                                    rotation: Rotation3::new(Vector3::from_element(0.0)),
                                },
                            ));
                        }
                        if block.face.contains(BlockFace::BOTTOM) {
                            self.raw_face_instances.push(RawFaceInstance::from(
                                block,
                                &Transform {
                                    translation: Translation3::from(
                                        Vector3::new(xx, yy, zz - Block::HALF_SIZE)
                                            + self.world_coord,
                                    ),
                                    rotation: Rotation3::new(Vector3::new(
                                        0.0,
                                        std::f32::consts::PI,
                                        0.0,
                                    )),
                                },
                            ));
                        }
                    }
                }
            }
        }
    }

    pub fn get_raw_face_instances(&self) -> Vec<RawFaceInstance> {
        self.raw_face_instances.clone()
    }
}
