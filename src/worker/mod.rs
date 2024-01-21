use bevy_ecs::prelude::*;
use bevy_tasks::{AsyncComputeTaskPool as BevyAsyncComputeTaskPool, TaskPool};
use std::ops::Deref;
use rapier3d::crossbeam::epoch::Pointable;

pub mod plugins;

#[derive(Resource)]
pub struct ComputeTaskPool(&'static BevyAsyncComputeTaskPool);

impl Deref for ComputeTaskPool {
    type Target = BevyAsyncComputeTaskPool;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

pub fn init_compute_task_pool() -> ComputeTaskPool {
    ComputeTaskPool(BevyAsyncComputeTaskPool::get())
}
