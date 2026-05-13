use crate::algebra::{Quaternion, Vec3};

pub struct Transform {
    translation: Vec3,
    rotation: Quaternion<f32>,
    scale: Vec3,
}

impl Transform {
    pub fn none() -> Self {
        Self {
            translation: Vec3::zeros(),
            rotation: Quaternion::<f32>::identity(),
            scale: Vec3::ones(),
        }
    }
}
