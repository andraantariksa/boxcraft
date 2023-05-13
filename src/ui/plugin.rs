use crate::game::schedule::ScheduleStage;
use crate::plugin::Plugin;
use crate::ui::systems::draw_ui::draw_ui;
use crate::ui::systems::pre_update::pre_update;
use crate::ui::UI;
use bevy_ecs::prelude::*;
use winit::window::Window;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn register_init(&self, world: &mut World, _schedule: &mut Schedule, window: &Window) {
        world.insert_resource(UI::new(window));
    }

    fn register_runtime(&self, _world: &mut World, schedule: &mut Schedule) {
        schedule
            .add_system(pre_update.in_set(ScheduleStage::PreUpdate))
            .add_system(draw_ui.in_set(ScheduleStage::PreRender));
    }
}
