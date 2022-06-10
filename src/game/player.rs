use crate::game::transform::Transform;
use crate::renderer::camera::CameraRenderer;
use crate::InputManager;
use legion::{system, World};
use nalgebra::{Rotation3, Vector, Vector3};
use std::time::Duration;

pub struct Player {
    flying: bool,
}

impl Player {
    pub fn new() -> Self {
        Self { flying: true }
    }
}

#[system(for_each)]
fn update_player(
    player: &Player,
    #[resource] camera: &CameraRenderer,
    #[resource] input_manager: &InputManager,
) {
}
