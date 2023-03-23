use crate::game::camera::Camera;

use crate::misc::input::InputManager;

use legion::system;

use std::time::Duration;
use winit::event::VirtualKeyCode;

pub struct Player {
    flying: bool,
}

impl Player {
    pub fn new() -> Self {
        Self { flying: false }
    }
}

#[system(for_each)]
pub fn update_player(
    _player: &Player,
    #[resource] camera: &mut Camera,
    #[resource] input_manager: &InputManager,
    #[resource] elapsed_time: &Duration,
) {
    const SPEED_MOVEMENT: f32 = 100.0;

    let delta_movement = SPEED_MOVEMENT * elapsed_time.as_secs_f32();

    if input_manager.is_key_pressed(&VirtualKeyCode::A) {
        camera.position -= delta_movement * camera.get_direction_right_horizontally();
    } else if input_manager.is_key_pressed(&VirtualKeyCode::D) {
        camera.position += delta_movement * camera.get_direction_right_horizontally();
    }

    if input_manager.is_key_pressed(&VirtualKeyCode::W) {
        camera.position += delta_movement * camera.get_direction_horizontally();
    } else if input_manager.is_key_pressed(&VirtualKeyCode::S) {
        camera.position -= delta_movement * camera.get_direction_horizontally();
    }

    if input_manager.is_key_pressed(&VirtualKeyCode::Space) {
        camera.position += delta_movement * Camera::WORLD_UP;
    } else if input_manager.is_key_pressed(&VirtualKeyCode::LControl) {
        camera.position -= delta_movement * Camera::WORLD_UP;
    }
}
