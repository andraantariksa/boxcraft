use crate::app::input::InputManager;
use crate::game::ScheduleStage;
use crate::plugin::Plugin;
use bevy_ecs::prelude::*;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn register_init(&self, world: &mut World, init_schedule: &mut Schedule) {
        world.insert_resource(InputManager::new());
    }

    fn register_runtime(&self, world: &mut World, schedule: &mut Schedule) {
        // fn clear(mut input_manager: ResMut<InputManager>) {
        //     input_manager.clear();
        // }
        //
        // schedule.add_system(
        //     clear
        //         .after(ScheduleStage::Render)
        //         .after(ScheduleStage::Update),
        // );
    }
}
