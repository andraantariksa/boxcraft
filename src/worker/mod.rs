use bevy_ecs::prelude::*;
use bevy_tasks::{ComputeTaskPool as BevyComputeTaskPool, TaskPool};
use std::ops::Deref;

pub mod plugins;

#[derive(Resource)]
pub struct ComputeTaskPool(&'static BevyComputeTaskPool);

impl Deref for ComputeTaskPool {
    type Target = BevyComputeTaskPool;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

pub fn init_compute_task_pool() -> ComputeTaskPool {
    ComputeTaskPool(BevyComputeTaskPool::init(TaskPool::new))
}
