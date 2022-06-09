use nalgebra::{Matrix4, Quaternion};

pub struct Transform {
    pub rotation: Quaternion<f32>,
    pub translation: Matrix4<f32>,
    // Doesn't really need scale for now
}
