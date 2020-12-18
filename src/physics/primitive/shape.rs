use std::fmt::Debug;

use crate::{math::Vec2, scene::Context};

pub trait Shape : Debug {
    fn center(&self) -> Vec2;
    fn support(&self, direction: Vec2) -> Vec2;
    fn draw(&self, position: Vec2, context: &mut Context);
}
