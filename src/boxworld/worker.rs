use crate::boxworld::chunk::Chunk;
use bevy_ecs::prelude::*;
use bevy_tasks::Task;
use nalgebra::Vector2;
use std::collections::HashMap;
use std::ops::Deref;

pub struct BoxWorldTaskResult {
    pub coord: Vector2<i32>,
    pub chunk: Chunk,
}

#[derive(Component)]
pub struct BoxWorldTask(pub Task<BoxWorldTaskResult>);

impl Deref for BoxWorldTask {
    type Target = Task<BoxWorldTaskResult>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
