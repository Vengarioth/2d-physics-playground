use crate::{math::*, scene::Context};

use super::Shape;

#[derive(Debug, Clone)]
pub struct Triangle {
    a: Vec2,
    b: Vec2,
    c: Vec2,
}

impl Triangle {
    pub fn new(a: Vec2, b: Vec2, c: Vec2) -> Self {
        Self {
            a,
            b,
            c,
        }
    }

    /// Returns the point furthest along `direction`
    pub fn support(&self, direction: Vec2) -> Vec2 {
        let d1 = self.a.dot(direction);
        let d2 = self.b.dot(direction);
        let d3 = self.c.dot(direction);

        if d1 > d2 {
            if d1 > d3 {
                self.a
            } else {
                self.c
            }
        } else {
            if d2 > d3 {
                self.b
            } else {
                self.c
            }
        }
    }

    pub fn center(&self) -> Vec2 {
        (self.a + self.b + self.c) / 3.0
    }

    pub fn draw(&self, position: Vec2, context: &mut Context) {
        context.draw_point(position + self.a);
        context.draw_point(position + self.b);
        context.draw_point(position + self.c);
        context.draw_line(position + self.a, position + self.b);
        context.draw_line(position + self.a, position + self.c);
        context.draw_line(position + self.b, position + self.c);
    }
}

impl Shape for Triangle {
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
