use crate::game::world::block::Block;

pub mod block;
pub mod generator;

pub struct World {
    render_chunk: i32,
    visible_chunks: Vec<Vec<Vec<Block>>>,
}

impl World {
    pub fn new() -> Self {
        Self {
            render_chunk: 50,
            visible_chunks: vec![],
        }
    }
}
