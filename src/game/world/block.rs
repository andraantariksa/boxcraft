use legion::system;
use std::time::Duration;

pub struct Block {
    r#type: BlockType,
}

#[repr(u32)]
pub enum BlockType {
    Dirt,
    Grass,
    Cobblestone,
}

#[system(for_each)]
fn update_chunk(block: &mut Block, #[resource] time_elapsed: &Duration) {}
