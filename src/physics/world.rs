use crate::scene::*;
use super::{Mass, RigidBody};

pub const INV_TIMESTEP: f32 = 60.0;
pub const TIMESTEP: f32 = 1.0 / INV_TIMESTEP;

#[derive(Debug)]
pub struct World {
    rigid_bodies: Vec<RigidBody>,
}

impl World {
    pub fn new() -> Self {
        Self {
            rigid_bodies: Vec::new(),
        }
    }

    pub fn add_rigid_body(&mut self, rigid_body: RigidBody) {
        self.rigid_bodies.push(rigid_body);
    }

    pub fn update(&mut self) {
        for rb in &mut self.rigid_bodies {
            if let Mass::Finite(inverse_mass) = rb.get_inverse_mass() {
                rb.update_forces();

                let acceleration = rb.force * inverse_mass;
                rb.velocity += acceleration * TIMESTEP;

                rb.constrain_velocity();

                rb.position += rb.velocity * TIMESTEP;
            }
        }
    }

    pub fn draw(&self, context: &mut Context) {
        for rb in &self.rigid_bodies {
            rb.draw(context);
        }
    }
}
