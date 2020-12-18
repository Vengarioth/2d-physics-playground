use crate::{math::*, scene::Context};

use super::Shape;

#[derive(Debug, Clone)]
pub struct Circle {
    pub center: Vec2,
    pub radius: f32,
}

impl Circle {
    pub fn new(center: Vec2, radius: f32) -> Self {
        Self {
            center,
            radius,
        }
    }

    /// Returns the point furthest along `direction`
    pub fn support(&self, direction: Vec2) -> Vec2 {
        self.center + (direction.normalize() * self.radius)
    }

    pub fn center(&self) -> Vec2 {
        self.center
    }

    pub fn draw(&self, position: Vec2, context: &mut Context) {
        context.draw_point(self.center + position);
        context.stroke_circle(self.center + position, self.radius);
    }
}

impl Shape for Circle {
    fn center(&self) -> Vec2 {
        self.center()
    }

    fn support(&self, direction: Vec2) -> Vec2 {
        self.support(direction)
    }

    fn draw(&self, position: Vec2, context: &mut Context) {
        self.draw(position, context);
    }
}
