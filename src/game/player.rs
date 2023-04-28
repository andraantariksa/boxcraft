use crate::game::camera::Camera;

use crate::misc::input::InputManager;

use bevy_ecs::prelude::*;
use winit::event::VirtualKeyCode;
use crate::game::systems::ElapsedTime;

#[derive(Resource)]
pub struct Player {
    flying: bool,
}

impl Player {
    pub fn new() -> Self {
        Self { flying: false }
    }
}

pub fn update_player(
    player: Res<Player>,
    mut camera: ResMut<Camera>,
    input_manager: Res<InputManager>,
    elapsed_time: Res<ElapsedTime>,
) {
    const SPEED_MOVEMENT: f32 = 100.0;

    let delta_movement = SPEED_MOVEMENT * elapsed_time.0;
    let right_direction = camera.get_direction_right_horizontally();
    let horizontal_direction = camera.get_direction_horizontally();

    if input_manager.is_key_pressed(&VirtualKeyCode::A) {
        camera.position -= delta_movement * right_direction;
    } else if input_manager.is_key_pressed(&VirtualKeyCode::D) {
        camera.position += delta_movement * right_direction;
    }

    if input_manager.is_key_pressed(&VirtualKeyCode::W) {
        camera.position += delta_movement * horizontal_direction;
    } else if input_manager.is_key_pressed(&VirtualKeyCode::S) {
        camera.position -= delta_movement * horizontal_direction;
    }

    if input_manager.is_key_pressed(&VirtualKeyCode::Space) {
        camera.position += delta_movement * Camera::WORLD_UP;
    } else if input_manager.is_key_pressed(&VirtualKeyCode::LControl) {
        camera.position -= delta_movement * Camera::WORLD_UP;
    }
}
