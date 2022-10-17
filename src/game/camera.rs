use crate::game::world::chunk::Chunk;
use crate::renderer::camera::CameraBuffer;
use nalgebra::{clamp, Matrix4, Perspective3, Point3, Vector2, Vector3};
use std::time::Duration;

#[derive(Default)]
pub struct Camera {
    pub position: Point3<f32>,
    yaw: f32,
    pitch: f32,
    sensitivity: f32,
}

impl Camera {
    pub const WORLD_UP: Vector3<f32> = Vector3::new(0.0, 1.0, 0.0);

    pub fn new() -> Self {
        Self {
            position: Point3::from_slice(&[
                Chunk::CHUNK_SIDE_SIZE / 2.0,
                5.0,
                Chunk::CHUNK_SIDE_SIZE / 2.0,
            ]),
            yaw: -90.0,
            pitch: 0.0,
            sensitivity: 1.0,
        }
    }

    pub fn get_direction(&self) -> Vector3<f32> {
        let yaw_cos = self.yaw.to_radians().cos();
        let yaw_sin = self.yaw.to_radians().sin();
        let pitch_cos = self.pitch.to_radians().cos();
        let pitch_sin = self.pitch.to_radians().sin();

        Vector3::new(yaw_cos * pitch_cos, pitch_sin, yaw_sin * pitch_cos).normalize()
    }

    pub fn get_direction_horizontally(&self) -> Vector3<f32> {
        let yaw_cos = self.yaw.to_radians().cos();
        let yaw_sin = self.yaw.to_radians().sin();
        let pitch_cos = self.pitch.to_radians().cos();

        Vector3::new(yaw_cos * pitch_cos, 0.0, yaw_sin * pitch_cos).normalize()
    }

    pub fn get_direction_right_horizontally(&self) -> Vector3<f32> {
        self.get_direction_horizontally()
            .cross(&Self::WORLD_UP)
            .normalize()
    }

    pub fn move_by_offset(&mut self, offset: &Vector2<f32>, time_elapsed: &Duration) {
        let time_elapsed_sec = time_elapsed.as_secs_f32();
        let timed_offset = offset * time_elapsed_sec * 10.0;

        self.yaw -= timed_offset.x;
        self.pitch += timed_offset.y;
        self.pitch = clamp(self.pitch, -89.0, 89.0);
    }

    pub fn get_view_matrix(&self) -> Matrix4<f32> {
        let direction = self.get_direction();
        Matrix4::look_at_rh(
            &self.position,
            &(&self.position + direction),
            &Self::WORLD_UP,
        )
    }

    pub fn get_projection_matrix(&self, aspect_ratio: f32) -> Matrix4<f32> {
        Perspective3::new(aspect_ratio, 90.0f32.to_radians(), 0.1, 1000.0).to_homogeneous()
    }

    pub fn get_yaw_pitch(&self) -> (f32, f32) {
        (self.yaw, self.pitch)
    }

    pub fn get_raw_buffer(&self, aspect_ratio: f32) -> CameraBuffer {
        CameraBuffer {
            projection: self.get_projection_matrix(aspect_ratio),
            view: self.get_view_matrix(),
            position: self.position,
            _p0: 0.0,
        }
    }
}
