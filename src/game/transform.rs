use nalgebra::{Matrix4, Quaternion, Rotation3, Vector, Vector3};

pub struct Transform {
    pub rotation: Rotation3<f32>,
    pub translation: Vector3<f32>,
    // Doesn't really need scale for now
}
