use crate::math::Vec2;

#[derive(Debug)]
pub struct Capsule {
    center: Vec2,
    height: f32,
    radius: f32,
}

impl Capsule {
    pub fn new(center: Vec2, height: f32, radius: f32) -> Self {
        Self {
            center,
            height,
            radius,
        }
    }
}
