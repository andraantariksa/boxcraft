use nalgebra::{Matrix4, Rotation3, Translation3};

pub struct Transform {
    pub rotation: Rotation3<f32>,
    pub translation: Translation3<f32>,
    // Doesn't really need scale for now
}

impl Transform {
    pub fn get_transformation_matrix(&self) -> Matrix4<f32> {
        self.translation.to_homogeneous() * self.rotation.to_homogeneous()
    }
}
