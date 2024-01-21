use crate::plugin::Plugin;
use crate::renderer::systems::wireframe::update_switch_wireframe;
use bevy_ecs::prelude::{Schedule, World};

pub struct RendererPlugin;

impl Plugin for RendererPlugin {
    fn register_runtime(&self, _world: &mut World, schedule: &mut Schedule) {
        schedule.add_systems(update_switch_wireframe);
    }
}
