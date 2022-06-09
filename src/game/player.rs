use crate::renderer::camera::Camera;
use std::time::Duration;

pub struct Player {
    flying: bool,
    camera: Camera,
}

impl Player {
    pub fn new() -> Self {
        Self {
            flying: true,
            camera: Camera::new(),
        }
    }

    pub fn update(&mut self, time_elapsed: Duration) {}
}
