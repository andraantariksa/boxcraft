use nalgebra::{Point3, Vector2};
use std::time::Duration;

#[derive(Default)]
pub struct Camera {
    position: Point3<f32>,
    yaw: f32,
    pitch: f32,
    sensitivity: f32,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            position: Point3::from_slice(&[0.0, 0.0, 0.0]),
            yaw: 0.0,
            pitch: 0.0,
            sensitivity: 1.0,
        }
    }

    pub fn move_by_offset(&mut self, offset: Vector2<f32>, time_elapsed: Duration) {}
}
