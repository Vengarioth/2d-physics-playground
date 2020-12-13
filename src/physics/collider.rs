use crate::{math::*, physics::primitive::*, scene::Context};

#[derive(Debug)]
pub enum Collider {
    Circle(Circle),
}

impl Collider {
    pub fn draw(&self, position: Vec2, context: &mut Context) {
        match self {
            Collider::Circle(circle) => circle.draw(position, context),
        }
    } 
}
