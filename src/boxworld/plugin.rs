use crate::boxworld::systems::calculate::{calculate, poll_calculation_task};
use crate::boxworld::systems::sync_camera::sync_camera;
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
            .add_systems((poll_calculation_task, calculate))
            .add_system(sync_camera.in_set(ScheduleStage::PostUpdate));
    }
}
