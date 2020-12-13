use crate::math::*;

#[derive(Debug)]
pub struct Camera {
    offset: Vec2,
    scale: f32,
}

impl Camera {
    pub fn new(offset: Vec2, scale: f32) -> Self {
        Self {
            offset,
            scale,
        }
    }

    pub fn transform_scalar(&self, scalar: f32) -> f32 {
        self.scale * scalar
    }

    pub fn transform(&self, point: Vec2) -> Vec2 {
        self.offset + (point * Vec2::new(self.scale, -self.scale))
    }
}
