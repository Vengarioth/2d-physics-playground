use crate::{math::*, scene::Context};

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

    pub fn draw(&self, position: Vec2, context: &mut Context) {
        context.set_color(context.blue());
        context.draw_circle(self.center + position, self.radius);
    }
}
