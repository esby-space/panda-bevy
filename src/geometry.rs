pub use glam::Vec2 as Vector;

use crate::Canvas;

pub struct Rectangle {
    pub point: Vector,
    pub size: Vector,
}

impl Rectangle {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self {
            point: Vector::new(x, y),
            size: Vector::new(w, h),
        }
    }

    pub fn intersects(&self, other: &Rectangle) -> bool {
        let (left1, top1, right1, bottom1) = self.bounds();
        let (left2, top2, right2, bottom2) = other.bounds();
        top1 < bottom2 && bottom1 > top2 && left1 < right2 && right1 > left2
    }

    // left, top, right, bottom
    pub fn bounds(&self) -> (f32, f32, f32, f32) {
        (
            self.point.x,
            self.point.y,
            self.point.x + self.size.x,
            self.point.y + self.size.y,
        )
    }

    pub fn draw(&self, canvas: &mut Canvas, pixel: [u8; 4]) {
        let point = self.point.round().as_ivec2();
        let size = self.size.round().as_ivec2();
        canvas.draw_rectangle(point.x, point.y, size.x, size.y, pixel);
    }
}

pub struct Circle {
    pub center: Vector,
    pub radius: f32
}

impl Circle {
    pub fn new(x: f32, y: f32, radius: f32) -> Self {
        Self {
            center: Vector::new(x, y),
            radius
        }
    }

    pub fn intersects(&self, other: &Circle) -> bool {
        (self.center - other.center).length_squared() < (self.radius + other.radius).powi(2)
    }

    pub fn draw(&self, canvas: &mut Canvas, pixel: [u8; 4]) {
        let center = self.center.round().as_ivec2();
        let radius = self.radius.round() as i32;
        canvas.draw_circle(center.x, center.y, radius, pixel);
    }
}

