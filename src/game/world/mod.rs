pub mod generator;

use crate::game::block::Block;

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
