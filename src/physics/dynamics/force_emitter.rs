use std::fmt::Debug;
use crate::{math::Vec2, physics::RigidBody, scene::Context};

pub trait ForceEmitter : Debug {
    fn evaluate(&self) -> Vec2;
    fn draw(&self, context: &mut Context, rigid_body: &RigidBody);
}

#[derive(Debug)]
pub struct GravityForceEmitter {
    gravity: Vec2,
}

impl GravityForceEmitter {
    pub fn new(gravity: Vec2) -> Self {
        Self {
            gravity,
        }
    }
}

impl ForceEmitter for GravityForceEmitter {
    fn evaluate(&self) -> Vec2 {
        self.gravity
    }

    fn draw(&self, _context: &mut Context, _rigid_body: &RigidBody) {
    }
}
