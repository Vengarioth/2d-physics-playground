use std::fmt::Debug;
use crate::{math::Vec2, scene::Context};

mod force_emitter;

pub use force_emitter::*;

use super::RigidBody;

pub trait VelocityConstraint : Debug {
    fn evaluate(&self, position: Vec2, velocity: Vec2) -> Vec2;
    fn draw(&self, context: &mut Context, rigid_body: &RigidBody);
}

pub fn position_constraint(p: Vec2, p0: Vec2) -> f32 {
    (p - p0).length()
}

/// A constraint that keeps the rigid body a a distance (radius) from a point (center)
#[derive(Debug)]
pub struct DistanceVelocityConstraint {
    center: Vec2,
    radius: f32,
}

impl DistanceVelocityConstraint {
    pub fn new(center: Vec2, radius: f32) -> Self {
        Self {
            center,
            radius,
        }
    }
}

impl VelocityConstraint for DistanceVelocityConstraint {
    fn evaluate(&self, position: Vec2, velocity: Vec2) -> Vec2 {
        let next_position = position + (velocity * super::TIMESTAMP);
        let direction = (next_position - self.center).normalize();
        let closest_point = self.center + (direction * self.radius);

        let correction = (closest_point - (position + (velocity * super::TIMESTAMP))) * super::INV_TIMESTAMP;

        velocity + correction
    }

    fn draw(&self, context: &mut Context, rigid_body: &RigidBody) {
        let position = rigid_body.position;
        let velocity = rigid_body.velocity;

        let next_position = position + (velocity * super::TIMESTAMP);
        let direction = (next_position - self.center).normalize();
        let closest_point = self.center + (direction * self.radius);

        let correction = (closest_point - (position + (velocity * super::TIMESTAMP))) * super::INV_TIMESTAMP;

        context.set_color(context.red());
        context.draw_point(self.center);
        context.stroke_circle(self.center, self.radius);
        context.draw_arrow(rigid_body.position, closest_point);
        
        context.set_color(context.green());
        context.draw_arrow(rigid_body.position, rigid_body.position + correction);
    }
}

#[derive(Debug)]
pub struct LineConstraint {
    a: Vec2,
    b: Vec2,
}

impl LineConstraint {
    pub fn new(a: Vec2, b: Vec2) -> Self {
        Self {
            a,
            b,
        }
    }
}

impl VelocityConstraint for LineConstraint {
    fn evaluate(&self, position: Vec2, velocity: Vec2) -> Vec2 {
        let next_position = position + (velocity * super::TIMESTAMP);

        let direction = (self.b - self.a).normalize();
        let v = (next_position - self.a).dot(direction).max(0.0).min((self.b - self.a).length());
        let closest_point = self.a + direction * v;

        let correction = (closest_point - (position + (velocity * super::TIMESTAMP))) * super::INV_TIMESTAMP;

        velocity + correction
    }

    fn draw(&self, context: &mut Context, rigid_body: &RigidBody) {
        let position = rigid_body.position;
        let velocity = rigid_body.velocity;
        let next_position = position + (velocity * super::TIMESTAMP);

        let direction = (self.b - self.a).normalize();
        let v = (next_position - self.a).dot(direction).max(0.0).min((self.b - self.a).length());
        let closest_point = self.a + direction * v;

        context.set_color(context.red());
        context.draw_line(self.a, self.b);
        context.draw_point(self.a);
        context.draw_point(self.b);
        
        context.set_color(context.green());
        context.draw_point(closest_point);
    }
}
