use nalgebra::{clamp, Matrix4, Perspective3, Point3, Vector2, Vector3};
use std::time::Duration;

#[derive(Default)]
pub struct Camera {
    position: Point3<f32>,
    _p1: [i32; 1],
    yaw: f32,
    pitch: f32,
    sensitivity: f32,
    _p2: [i32; 1],
}

impl Camera {
    const UP: Vector3<f32> = Vector3::new(0.0, 1.0, 0.0);
    // const FRONT: Vector3<f32> = Vector3::new(0.0, 0.0, -1.0);

    pub fn new() -> Self {
        Self {
            position: Point3::from_slice(&[0.0, 0.0, 0.0]),
            yaw: 0.0,
            pitch: 0.0,
            sensitivity: 1.0,
            _p1: [0],
            _p2: [0],
        }
    }

    pub fn move_by_offset(&mut self, offset: &Vector2<f32>, time_elapsed: &Duration) {
        let time_elapsed_sec = time_elapsed.as_secs_f32();
        let timed_offset = offset * time_elapsed_sec;

        self.yaw += timed_offset.x;
        self.pitch += timed_offset.y;
        self.pitch = clamp(self.pitch, -89.0, 89.0);
    }

    pub fn get_view_matrix(&self) -> Matrix4<f32> {
        let yaw_cos = self.yaw.to_radians().cos();
        let yaw_sin = self.yaw.to_radians().sin();
        let pitch_cos = self.pitch.to_radians().cos();
        let pitch_sin = self.pitch.to_radians().sin();

        let direction: Vector3<f32> =
            Vector3::new(yaw_cos * pitch_cos, pitch_sin, yaw_sin * pitch_cos).normalize();

        Matrix4::look_at_rh(&self.position, &(&self.position + direction), &Self::UP)
    }

    pub fn get_projection_matrix(&self) -> Matrix4<f32> {
        Perspective3::new(16.0 / 9.0, 90.0f32.to_radians(), 1.0, 100000.0).to_homogeneous()
    }
}
