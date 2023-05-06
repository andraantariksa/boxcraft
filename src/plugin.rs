use crate::game::Game;
use bevy_ecs::prelude::*;

pub trait Plugin {
    fn register_init(&self, world: &mut World, init_schedule: &mut Schedule) {}
    fn register_runtime(&self, world: &mut World, schedule: &mut Schedule) {}
}
