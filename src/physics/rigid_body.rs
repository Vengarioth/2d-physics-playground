use std::fmt::Debug;

use crate::{math::*, scene::Context};
use super::{Collider, dynamics::{ForceEmitter, VelocityConstraint}};

pub const SOLVER_ITERATIONS: usize = 4;

#[derive(Debug, Copy, Clone)]
pub enum Mass {
    Infinite,
    Finite(f32),
}

#[derive(Debug)]
pub struct RigidBody {
    pub velocity: Vec2,
    pub force: Vec2,
    pub position: Vec2,
    pub mass: Mass,
    pub cor: f32,
    pub collider: Collider,

    force_emitters: Vec<Box<dyn ForceEmitter>>,
    velocity_constraints: Vec<Box<dyn VelocityConstraint>>,
}

impl RigidBody {
    pub fn new(position: Vec2, mass: Mass, cor: f32, collider: Collider) -> Self {
        Self {
            velocity: Vec2::zero(),
            force: Vec2::zero(),
            position,
            mass,
            cor,
            collider,

            force_emitters: Vec::new(),
            velocity_constraints: Vec::new(),
        }
    }

    pub fn add_force_emitter(&mut self, emitter: Box<dyn ForceEmitter>) {
        self.force_emitters.push(emitter);
    }

    pub fn add_velocity_constraint(&mut self, constraint: Box<dyn VelocityConstraint>) {
        self.velocity_constraints.push(constraint);
    }

    pub fn get_cor(&self) -> f32 {
        self.cor
    }

    pub fn get_inverse_mass(&self) -> Mass {
        match self.mass {
            Mass::Infinite => Mass::Infinite,
            Mass::Finite(mass) => Mass::Finite(1.0 / mass),
        }
    }

    pub fn update_forces(&mut self) {
        self.force = Vec2::zero();
        for emitter in &self.force_emitters {
            self.force += emitter.evaluate();
        }
    }

    pub fn constrain_velocity(&mut self) {
        let mut velocity = self.velocity;
        for _ in 0..SOLVER_ITERATIONS {
            for constraint in &self.velocity_constraints {
                let next_position = self.position + (velocity * super::TIMESTEP);
                let closest_point = constraint.evaluate(next_position, velocity);
                let correction = (closest_point - next_position) * super::INV_TIMESTEP;
                velocity += correction;
            }
        }
        self.velocity = velocity;
    }

    pub fn draw(&self, context: &mut Context) {
        context.set_color(context.blue());
        self.collider.draw(self.position, context);

        context.set_color(context.yellow());
        context.draw_arrow(self.position, self.position + self.velocity);

        for emitter in &self.force_emitters {
            emitter.draw(context, &self);
        }

        for constraint in &self.velocity_constraints {
            constraint.draw(context, &self);
        }
    }
}
