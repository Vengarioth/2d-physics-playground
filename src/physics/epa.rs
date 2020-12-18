use crate::math::Vec2;

use super::primitive::Shape;

/// The Expanding Polytope Algorithm
pub fn epa(shape_a: impl Shape, shape_b: impl Shape, vertices: Vec<Vec2>, context: &mut crate::scene::Context) {
    let origin = Vec2::zero();

    context.set_color(context.yellow());
    context.draw_line(vertices[0], vertices[1]);
    context.draw_line(vertices[1], vertices[2]);
    context.draw_line(vertices[2], vertices[0]);

    let mut closest_distance = std::f32::INFINITY;
    let mut closest_normal = Vec2::zero();
    let mut closest_index = 0;
    let mut line = Vec2::zero();

    for i in 0..vertices.len() {
        let j = (i + 1) % vertices.len();

        let line = vertices[j] - vertices[i];
        let normal = line.perpendicular_cw().normalize();

        context.draw_normal(vertices[i] + ((vertices[j] - vertices[i]) / 2.0), normal);
    }
}

fn find_closest_edge() {
    
}
