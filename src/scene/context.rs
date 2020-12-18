use web_sys::CanvasRenderingContext2d;
use crate::math::*;
use super::{Camera, Color};

#[derive(Debug)]
pub struct Context {
    inner: CanvasRenderingContext2d,
    camera: Camera,
}

impl Context {
    pub fn new(inner: CanvasRenderingContext2d, camera: Camera) -> Self {
        Self {
            inner,
            camera,
        }
    }

    pub fn clear(&mut self) {
        self.inner.clear_rect(0.0, 0.0, 1280.0, 720.0);
    }

    pub fn red(&self) -> Color {
        Color::rgb(236, 59, 18)
    }

    pub fn green(&self) -> Color {
        Color::rgb(37, 214, 48)
    }

    pub fn blue(&self) -> Color {
        Color::rgb(27, 101, 244)
    }

    pub fn yellow(&self) -> Color {
        Color::rgb(244, 210, 27)
    }

    pub fn white(&self) -> Color {
        Color::rgb(255, 255, 255)
    }

    pub fn black(&self) -> Color {
        Color::rgb(0, 0, 0)
    }

    pub fn set_color(&mut self, color: Color) {
        let color = color.into();
        self.inner.set_fill_style(&color);
        self.inner.set_stroke_style(&color);
    }

    pub fn set_fill_color(&mut self, color: Color) {
        self.inner.set_fill_style(&color.into());
    }

    pub fn set_stroke_color(&mut self, color: Color) {
        self.inner.set_stroke_style(&color.into());
    }

    pub fn draw_point(&mut self, point: Vec2) {
        let point = self.camera.transform(point);
        self.inner.begin_path();
        self.inner.move_to(point.x as f64, point.y as f64);
        self.inner.arc(point.x as f64, point.y as f64, 3.0, 2.0 * std::f64::consts::PI, 0.0).unwrap();
        self.inner.fill();
        self.inner.stroke();
        self.inner.close_path();
    }

    pub fn draw_circle(&mut self, center: Vec2, radius: f32) {
        let center = self.camera.transform(center);
        let radius = self.camera.transform_scalar(radius);
        self.inner.begin_path();
        self.inner.move_to(center.x as f64, center.y as f64);
        self.inner.arc(center.x as f64, center.y as f64, radius as f64, 2.0 * std::f64::consts::PI, 0.0).unwrap();
        self.inner.fill();
        self.inner.stroke();
        self.inner.close_path();
    }

    pub fn stroke_circle(&mut self, center: Vec2, radius: f32) {
        let center = self.camera.transform(center);
        let radius = self.camera.transform_scalar(radius);
        self.inner.begin_path();
        self.inner.move_to((center.x + radius) as f64, center.y as f64);
        self.inner.arc(center.x as f64, center.y as f64, radius as f64, 2.0 * std::f64::consts::PI, 0.0).unwrap();
        self.inner.stroke();
        self.inner.close_path();
    }

    pub fn draw_line(&mut self, a: Vec2, b: Vec2) {
        let a = self.camera.transform(a);
        let b = self.camera.transform(b);

        self.inner.begin_path();
        self.inner.move_to(a.x as f64, a.y as f64);
        self.inner.line_to(b.x as f64, b.y as f64);
        self.inner.stroke();
        self.inner.close_path();
    }

    pub fn draw_arrow(&mut self, from: Vec2, to: Vec2) {
        let from = self.camera.transform(from);
        let to = self.camera.transform(to);

        self.inner.begin_path();
        self.inner.move_to(from.x as f64, from.y as f64);
        self.inner.line_to(to.x as f64, to.y as f64);
        self.inner.stroke();
        self.inner.close_path();
    }

    pub fn draw_normal(&mut self, position: Vec2, normal: Vec2) {
        let from = self.camera.transform(position);
        let to = self.camera.transform(position + normal);

        self.inner.begin_path();
        self.inner.move_to(from.x as f64, from.y as f64);
        self.inner.line_to(to.x as f64, to.y as f64);
        self.inner.stroke();
        self.inner.close_path();
    }
}
