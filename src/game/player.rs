use crate::game::camera::Camera;
use crate::game::transform::Transform;
use crate::renderer::camera::CameraRenderer;
use crate::InputManager;
use legion::{system, World};
use nalgebra::{Rotation3, Vector, Vector3};
use std::time::Duration;
use winit::event::VirtualKeyCode;

pub struct Player {
    flying: bool,
}

impl Player {
    pub fn new() -> Self {
        Self { flying: true }
    }
}

#[system(for_each)]
pub fn update_player(
    player: &Player,
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
        camera.position += delta_movement * camera.get_direction();
    } else if input_manager.is_key_pressed(&VirtualKeyCode::S) {
        camera.position -= delta_movement * camera.get_direction();
    }

    if input_manager.is_key_pressed(&VirtualKeyCode::Space) {
        camera.position += delta_movement * Camera::WORLD_UP;
    } else if input_manager.is_key_pressed(&VirtualKeyCode::LControl) {
        camera.position -= delta_movement * Camera::WORLD_UP;
    }
}
