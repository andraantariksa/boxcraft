use crate::game::transform::Transform;
use crate::game::world::block::{Block, RawFaceInstance};
use nalgebra::{Rotation3, Translation3, Vector3};

use super::block::BlockFace;

#[derive(Clone)]
pub struct Chunk {
    blocks: Vec<Vec<Vec<Option<Block>>>>,
}

impl Chunk {
    pub const CHUNK_SIDE_BLOCK: usize = 16;
    pub const CHUNK_VERTICAL_BLOCK: usize = 2;
    pub const TOTAL_BLOCKS: usize =
        Self::CHUNK_SIDE_BLOCK * Self::CHUNK_SIDE_BLOCK * Self::CHUNK_VERTICAL_BLOCK;

    pub const CHUNK_SIDE_SIZE: f32 = Self::CHUNK_SIDE_BLOCK as f32 * Block::SIZE;
    pub const CHUNK_HALF_SIDE_SIZE: f32 = Self::CHUNK_SIDE_SIZE as f32 * 0.5;

    pub fn from(block: Option<Block>) -> Self {
        // Initialize faces
        let mut blocks = vec![
            vec![vec![block; Self::CHUNK_SIDE_BLOCK]; Self::CHUNK_VERTICAL_BLOCK];
            Self::CHUNK_SIDE_BLOCK
        ];

        // Calculate faces
        Self::calculate_faces(&mut blocks);

        Self { blocks }
    }

    fn calculate_faces(blocks: &mut Vec<Vec<Vec<Option<Block>>>>) {
        for x in 0..Self::CHUNK_SIDE_BLOCK {
            for y in 0..Self::CHUNK_VERTICAL_BLOCK {
                for z in 0..Self::CHUNK_SIDE_BLOCK {
                    let mut face = BlockFace::empty();
                    {
                        let y_blocks = unsafe { blocks.get_unchecked(x) };
                        let z_blocks = unsafe { y_blocks.get_unchecked(y) };
                        let maybe_block = unsafe { z_blocks.get_unchecked(z) };

                        if let Some(block) = maybe_block {
                            // X
                            {
                                let _x = x + 1;
                                if blocks.len() >= _x
                                    || unsafe {
                                        blocks.get_unchecked(_x).get_unchecked(y).get_unchecked(z)
                                    }
                                    .is_none()
                                {
                                    face |= BlockFace::RIGHT;
                                }
                            }
                            {
                                if x == 0
                                    || unsafe {
                                        blocks
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

                    let y_blocks = unsafe { blocks.get_unchecked_mut(x) };
                    let z_blocks = unsafe { y_blocks.get_unchecked_mut(y) };
                    let maybe_block = unsafe { z_blocks.get_unchecked_mut(z) };

                    if !face.is_empty() {
                        unsafe { maybe_block.as_mut().unwrap_unchecked() }.face |= face;
                    }
                }
            }
        }
    }

    pub fn get_raw_face_instances(
        &self,
        faces: &mut Vec<RawFaceInstance>,
        center_point: &Vector3<f32>,
    ) {
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
                            faces.push(RawFaceInstance::from(
                                block,
                                &Transform {
                                    translation: Translation3::from(
                                        Vector3::new(xx + Block::HALF_SIZE, yy, zz) + center_point,
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
                            faces.push(RawFaceInstance::from(
                                block,
                                &Transform {
                                    translation: Translation3::from(
                                        Vector3::new(xx - Block::HALF_SIZE, yy, zz) + center_point,
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
                            faces.push(RawFaceInstance::from(
                                block,
                                &Transform {
                                    translation: Translation3::from(
                                        Vector3::new(xx, yy + Block::HALF_SIZE, zz) + center_point,
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
                            faces.push(RawFaceInstance::from(
                                block,
                                &Transform {
                                    translation: Translation3::from(
                                        Vector3::new(xx, yy - Block::HALF_SIZE, zz) + center_point,
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
                            faces.push(RawFaceInstance::from(
                                block,
                                &Transform {
                                    translation: Translation3::from(
                                        Vector3::new(xx, yy, zz + Block::HALF_SIZE) + center_point,
                                    ),
                                    rotation: Rotation3::new(Vector3::new(0.0, 0.0, 0.0)),
                                },
                            ));
                        }
                        if block.face.contains(BlockFace::BOTTOM) {
                            faces.push(RawFaceInstance::from(
                                block,
                                &Transform {
                                    translation: Translation3::from(
                                        Vector3::new(xx, yy, zz - Block::HALF_SIZE) + center_point,
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
}
