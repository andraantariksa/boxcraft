use crate::boxworld::systems::sync_camera::sync_camera;
use crate::boxworld::systems::worker::{calculate, update_worker};
use crate::boxworld::BoxWorld;
use crate::game::schedule::ScheduleStage;
use crate::plugin::Plugin;
use bevy_ecs::prelude::*;
use winit::window::Window;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn register_init(&self, world: &mut World, schedule: &mut Schedule, window: &Window) {
        world.insert_resource(BoxWorld::new());
    }

    fn register_runtime(&self, world: &mut World, schedule: &mut Schedule) {
        schedule
            .add_systems((calculate, update_worker))
            .add_system(sync_camera.in_set(ScheduleStage::PostUpdate));
    }
}
