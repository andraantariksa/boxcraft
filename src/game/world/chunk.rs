use crate::game::transform::Transform;
use crate::game::world::block::{Block, FacesRawInstance};
use crate::game::world::World;
use nalgebra::{Rotation3, Translation3, Vector3};

#[derive(Clone)]
pub struct Chunk {
    blocks: Vec<Vec<Vec<Option<Block>>>>,
}

impl Chunk {
    pub const CHUNK_SIDE_SIZE: usize = 16;
    pub const CHUNK_VERTICAL_SIZE: usize = 256;

    pub fn from(block: Option<Block>) -> Self {
        let mut blocks = vec![
            vec![vec![block; Self::CHUNK_SIDE_SIZE]; Self::CHUNK_VERTICAL_SIZE];
            Self::CHUNK_SIDE_SIZE
        ];
        for y_blocks in blocks.iter_mut() {
            for (y, z_blocks) in y_blocks.iter_mut().enumerate() {
                if y > 120 {
                    for block in z_blocks.iter_mut() {
                        *block = None;
                    }
                }
            }
        }
        Self { blocks }
    }

    pub fn get_total_blocks() -> usize {
        Self::CHUNK_SIDE_SIZE * Self::CHUNK_SIDE_SIZE * Self::CHUNK_VERTICAL_SIZE
    }

    pub fn get_faces(&self, center_point: &Vector3<f32>) -> Vec<FacesRawInstance> {
        let mut blocks = Vec::<FacesRawInstance>::with_capacity(Self::get_total_blocks() * 6);
        for x in 0..Self::CHUNK_SIDE_SIZE {
            for y in 0..Self::CHUNK_VERTICAL_SIZE {
                for z in 0..Self::CHUNK_SIDE_SIZE {
                    let y_blocks = unsafe { self.blocks.get_unchecked(x) };
                    let z_blocks = unsafe { y_blocks.get_unchecked(y) };
                    let maybe_block = unsafe { z_blocks.get_unchecked(z) };

                    if let Some(block) = maybe_block {
                        let yy = -(Self::CHUNK_VERTICAL_SIZE as f32 * Block::SIZE / 2.0)
                            + Block::HALF_SIZE
                            + (Block::SIZE * y as f32);
                        let xx = -(Self::CHUNK_SIDE_SIZE as f32 * Block::SIZE / 2.0)
                            + Block::HALF_SIZE
                            + (Block::SIZE * x as f32);
                        let zz = -(Self::CHUNK_SIDE_SIZE as f32 * Block::SIZE / 2.0)
                            + Block::HALF_SIZE
                            + (Block::SIZE * z as f32);

                        // X
                        {
                            let y_blocks = self.blocks.get(x + 1);
                            if y_blocks.is_none() || y_blocks.unwrap()[y][z].is_none() {
                                blocks.push(FacesRawInstance::from(
                                    block,
                                    &Transform {
                                        translation: Translation3::new(
                                            xx + Block::HALF_SIZE,
                                            yy,
                                            zz,
                                        ),
                                        rotation: Rotation3::new(Vector3::new(
                                            0.0,
                                            std::f32::consts::FRAC_PI_2,
                                            0.0,
                                        )),
                                    },
                                ));
                            }
                        }
                        if x == 0 || self.blocks[x - 1][y][z].is_none() {
                            blocks.push(FacesRawInstance::from(
                                block,
                                &Transform {
                                    translation: Translation3::new(xx - Block::HALF_SIZE, yy, zz),
                                    rotation: Rotation3::new(Vector3::new(
                                        0.0,
                                        -std::f32::consts::FRAC_PI_2,
                                        0.0,
                                    )),
                                },
                            ));
                        }

                        // Y
                        {
                            let z_blocks = y_blocks.get(y + 1);
                            if z_blocks.is_none() || z_blocks.unwrap()[z].is_none() {
                                blocks.push(FacesRawInstance::from(
                                    block,
                                    &Transform {
                                        translation: Translation3::new(
                                            xx,
                                            yy + Block::HALF_SIZE,
                                            zz,
                                        ),
                                        rotation: Rotation3::new(Vector3::new(
                                            -std::f32::consts::FRAC_PI_2,
                                            0.0,
                                            0.0,
                                        )),
                                    },
                                ));
                            }
                        }
                        if y == 0 || y_blocks[y - 1][z].is_none() {
                            blocks.push(FacesRawInstance::from(
                                block,
                                &Transform {
                                    translation: Translation3::new(xx, yy - Block::HALF_SIZE, zz),
                                    rotation: Rotation3::new(Vector3::new(
                                        std::f32::consts::FRAC_PI_2,
                                        0.0,
                                        0.0,
                                    )),
                                },
                            ));
                        }

                        // Z
                        if z_blocks.get(z + 1).is_none() {
                            blocks.push(FacesRawInstance::from(
                                block,
                                &Transform {
                                    translation: Translation3::new(xx, yy, zz + Block::HALF_SIZE),
                                    rotation: Rotation3::new(Vector3::new(0.0, 0.0, 0.0)),
                                },
                            ));
                        }
                        if z == 0 || z_blocks[z - 1].is_none() {
                            blocks.push(FacesRawInstance::from(
                                block,
                                &Transform {
                                    translation: Translation3::new(xx, yy, zz - Block::HALF_SIZE),
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
        blocks
    }
}
