use crate::algebra::{Vec2, Vec3};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Vertex {
    position: Vec3,
    normal: Vec3,
    uv: Vec2,
}

impl Vertex {
    pub fn new(position: Vec3, normal: Vec3, uv: Vec2) -> Self {
        Self {
            position,
            normal,
            uv,
        }
    }
}
