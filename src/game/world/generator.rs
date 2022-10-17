use nalgebra::Vector2;

use crate::game::world::chunk::Chunk;

struct WorldGenerator;

impl WorldGenerator {
    pub fn new() -> Self {
        Self
    }

    // pub fn gen_with_center_point(&self, chunk_coord: Vector2<i32>) -> Chunk {
    //     let mut visible_chunks = VecDeque::with_capacity(World::RENDER_CHUNK * 2 + 1);
    //     Chunk::from(Some(Block::new(BlockType::Dirt)));
    // }
}
