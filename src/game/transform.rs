use crate::game::block::Block;
use crate::renderer::box_instance::BoxInstance;
use nalgebra::{Matrix4, Point2, Rotation3, Translation, Translation3, Vector3};

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
