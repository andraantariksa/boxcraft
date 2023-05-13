use crate::app::input::InputManager;
use bevy_ecs::prelude::*;

pub fn clear(mut input_manager: ResMut<InputManager>) {
    input_manager.clear();
}
