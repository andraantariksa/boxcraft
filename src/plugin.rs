use bevy_ecs::prelude::*;
use winit::window::Window;

pub trait Plugin {
    #[allow(unused_variables)]
    fn register_init(&self, world: &mut World, schedule: &mut Schedule, window: &Window) {}
    #[allow(unused_variables)]
    fn register_runtime(&self, world: &mut World, schedule: &mut Schedule) {}
}
