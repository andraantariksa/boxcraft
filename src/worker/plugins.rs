use crate::plugin::Plugin;
use crate::worker::init_compute_task_pool;
use bevy_ecs::prelude::*;
use winit::window::Window;

pub struct WorkerPlugin;

impl Plugin for WorkerPlugin {
    fn register_init(&self, world: &mut World, _schedule: &mut Schedule, _window: &Window) {
        // world.insert_resource(init_compute_task_pool());
    }
}
