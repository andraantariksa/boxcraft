use crate::app::input::InputManager;

use crate::app::input::systems::clear::clear;
use crate::game::schedule::ScheduleStage;
use crate::plugin::Plugin;
use bevy_ecs::prelude::*;
use winit::window::Window;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn register_init(&self, world: &mut World, _init_schedule: &mut Schedule, _window: &Window) {
        world.insert_resource(InputManager::new());
    }

    fn register_runtime(&self, _world: &mut World, schedule: &mut Schedule) {
        schedule.add_systems(clear.in_set(ScheduleStage::PreRender));
    }
}
