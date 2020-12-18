use crate::math::*;
use crate::physics::primitive::*;

/// Uses three consecutive cross products to obtain a perpendicular vector
fn tripple_product(a: Vec2, b: Vec2, c: Vec2) -> Vec2 {
    let ac = a.x * c.x + a.y * c.y;
    let bc = b.x * c.x + b.y * c.y;
    Vec2::new(b.x * ac - a.x * bc, b.y * ac - a.y * bc)
}

/// Returns true if both vectors point in the same direction
fn same_direction(ab: Vec2, a0: Vec2) -> bool {
    ab.dot(a0) > 0.0
}

/// Returns a new vertex on the Minkowski difference along `direction`
fn calculate_support(shape_a: &impl Shape, shape_b: &impl Shape, direction: Vec2) -> Vec2 {
    shape_a.support(direction) - shape_b.support(direction * -1.0)
}

/// The Gilbert–Johnson–Keerthi distance algorithm
/// Determines if two convex `Shape`s overlap by evolving a simplex of points on the Minkowski sum. The shapes overlap if a simplex can be found that encloses the origin.
/// The Minkowski sum is just the the sum of points of shape A minus the sum of points of shape B
/// A simplex is just a generalization of a Triangle, we only use the forms up to three vertices: Point, Line Segment and Triangle itself.
pub fn gjk(shape_a: impl Shape, shape_b: impl Shape, context: &mut crate::scene::Context) -> (bool, Vec<Vec2>) {

    // The vertices of our simplex
    let mut vertices = Vec::new();
    // The direction to search in
    let mut direction = shape_b.center() - shape_a.center();
    
    let s = calculate_support(&shape_a, &shape_b, direction);
    vertices.push(s);
    direction = -s;

    loop {
        // We add a point to the simplex on each iteration
        let a = calculate_support(&shape_a, &shape_b, direction);
        if a.dot(direction) < 0.0 {
            return (false, vertices);
        }

        vertices.push(a);

        match vertices.len() {
            2 => {
                let a = vertices[1];
                let b = vertices[0];

                let a0 = -a;
                let ab = b - a;

                if same_direction(ab, a0) {
                    // inside simplex
                    direction = tripple_product(ab, a0, ab).normalize();
                } else {
                    // outside simplex
                    vertices.remove(0); // remove b
                    direction = a0;
                }
            },
            3 => {
                let a = vertices[2];
                let b = vertices[1];
                let c = vertices[0];

                let a0 = -a;
                let ab = b - a;
                let ac = c - a;

                let ab_p = tripple_product(ac, ab, ab);
                let ac_p = tripple_product(ab, ac, ac);

                match (same_direction(ab_p, a0), same_direction(ac_p, a0)) {
                    (true, true) => {
                        // the origin is outside the enclosed area
                        vertices.remove(1); // remove b
                        vertices.remove(0); // remove c
                        direction = a0;
                    },
                    (false, true) => {
                        // the origin is outside, on the a -> b edge
                        vertices.remove(1); // remove b
                        direction = ab_p;
                    },
                    (true, false) => {
                        // the origin is outside, on the a -> c edge
                        vertices.remove(0); // remove c
                        direction = ac_p;
                    },
                    (false, false) => {
                        // the origin is within the enclosed area
                        
                        let bc = c - b;
                        let b0 = -b;
                        let bc_p = tripple_product(bc, ab, bc);

                        if same_direction(bc_p, b0) {
                            vertices.remove(2); // remove a
                            direction = bc_p;
                        } else {
                            // context.set_color(context.yellow());
                            // context.draw_line(a, b);
                            // context.draw_line(a, c);
                            // context.draw_line(b, c);
                            // context.set_color(context.green());
                            // context.draw_point(a);
                            // context.draw_point(b);
                            // context.draw_point(c);
                            // context.set_color(context.red());
                            // context.draw_normal(b + (c - b) / 2.0, bc_p.normalize());
                            // context.draw_normal(a + (b - a) / 2.0, ab_p.normalize());
                            // context.draw_normal(a + (c - a) / 2.0, ac_p.normalize());

                            return (true, vertices);
                        }
                    }
                }
            },
            _ => panic!("Wrong number of points on simplex"),
        }
    }
}
