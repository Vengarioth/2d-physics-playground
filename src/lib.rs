use std::{cell::RefCell, rc::Rc};

use math::Vec2;
use physics::{epa, gjk, primitive::{Circle, Triangle}};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

pub mod math;
pub mod scene;
pub mod physics;

#[macro_export]
macro_rules! log {
    ( $( $t:tt )* ) => {
        #[allow(unused_unsafe)]unsafe { web_sys::console::log_1(&format!( $( $t )* ).into()); }
    }
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let mut world = crate::physics::World::new();

    let mut rb = crate::physics::RigidBody::new(
        Vec2::new(-12.0, 12.0),
        crate::physics::Mass::Finite(1.0),
        1.0,
        crate::physics::Collider::Circle(crate::physics::primitive::Circle::new(Vec2::zero(), 1.0)),
    );
    // rb.add_velocity_constraint(Box::new(crate::physics::dynamics::DistanceVelocityConstraint::new(Vec2::zero(), 10.0)));
    rb.add_velocity_constraint(Box::new(crate::physics::dynamics::DistanceVelocityConstraint::new(Vec2::new(2.0, 2.0), 10.0)));
    // rb.add_velocity_constraint(Box::new(crate::physics::dynamics::LineConstraint::new(Vec2::new(-12.0, 12.0), Vec2::new(12.0, -12.0))));
    rb.add_force_emitter(Box::new(crate::physics::dynamics::GravityForceEmitter::new(Vec2::new(0.0, -10.0))));
    world.add_rigid_body(rb);

    let camera = crate::scene::Camera::new(Vec2::new(1280.0 / 2.0, 720.0 / 2.0), 25.0);

    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    let mut context = crate::scene::Context::new(context, camera);

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();
    let mut t: f32 = 0.0;

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        context.clear();
        world.update();
        world.draw(&mut context);

        /*
        context.set_color(context.black());
        context.draw_point(Vec2::zero());

        let offset = 3.0;
        let p = Vec2::new(offset + (1.0), 0.5);
        let a = Circle::new(p, 1.0);
        let b = Triangle::new(Vec2::new(offset + 0.0, -1.0), Vec2::new(offset + 1.0, -1.0), Vec2::new(offset + 0.5, 1.0));

        let (overlaps, vertices) = gjk(a.clone(), b.clone(), &mut context);
        if overlaps {
            epa(a.clone(), b.clone(), vertices, &mut context);
            context.set_color(context.red());
        } else {
            context.set_color(context.green());
        }
        a.draw(Vec2::zero(), &mut context);
        b.draw(Vec2::zero(), &mut context);
        */

        request_animation_frame(f.borrow().as_ref().unwrap());
        t += 1.0 / 60.0;
    }) as Box<dyn FnMut()>));
    request_animation_frame(g.borrow().as_ref().unwrap());

    Ok(())
}

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}
