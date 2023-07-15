use bevy_ecs::system::Resource;
use pixels::{wgpu::Color as WGPUColor, Pixels};

use crate::{
    geometry::Vec2,
    line::Bresenham,
};

#[derive(Resource)]
pub struct Canvas(pub Pixels);

impl Canvas {
    pub fn width(&self) -> u32 {
        self.0.texture().width()
    }

    pub fn height(&self) -> u32 {
        self.0.texture().height()
    }

    pub fn pixels(&mut self) -> &mut [u8] {
        self.0.frame_mut()
    }
}

impl Canvas {
    pub fn clear(&mut self, pixel: [u8; 4]) {
        for slice in self.pixels().chunks_mut(4) {
            slice.copy_from_slice(&pixel);
        }
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> [u8; 4] {
        let index = 4 * (y * self.width() + x) as usize;
        let slice = &self.0.frame()[index..index + 4];
        slice.try_into().unwrap()
    }

    pub fn draw_pixel(&mut self, x: i32, y: i32, pixel: [u8; 4]) {
        if x < 0 || x > self.width() as i32 - 1 || y < 0 || y > self.height() as i32 - 1 {
            return;
        };

        let index = (4 * (y * self.width() as i32 + x)) as usize;
        self.pixels()[index..(index + 4)].copy_from_slice(&pixel[..4]);
    }

    pub fn draw_nearest_pixel(&mut self, x: f32, y: f32, pixel: [u8; 4]) {
        let (x, y) = (x.round() as i32, y.round() as i32);
        self.draw_pixel(x, y, pixel);
    }

    pub fn draw_line(&mut self, start: &Vec2, end: &Vec2, pixel: [u8; 4]) {
        for point in Bresenham::new(start, end) {
            self.draw_nearest_pixel(point.x, point.y, pixel);
        }
    }

    pub fn draw_rectangle(&mut self, x: i32, y: i32, w: i32, h: i32, pixel: [u8; 4]) {
        for y in y..y + h {
            for x in x..x + w {
                self.draw_pixel(x, y, pixel);
            }
        }
    }

    pub fn draw_circle(&mut self, center_x: i32, center_y: i32, r: i32, pixel: [u8; 4]) {
        for y in center_y - r..center_y + r {
            for x in center_x - r..center_x + r {
                let dx = center_x - x;
                let dy = center_y - y;
                let distance = dx.pow(2) + dy.pow(2);

                if distance < r.pow(2) {
                    self.draw_pixel(x, y, pixel);
                }
            }
        }
    }
}

pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const WHITE: Self = Self {
        r: 255,
        g: 255,
        b: 255,
        a: 255,
    };

    pub const BLACK: Self = Self {
        r: 0,
        g: 0,
        b: 0,
        a: 255,
    };

    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub fn pixel(&self) -> [u8; 4] {
        [self.r, self.g, self.b, self.a]
    }
}

impl From<WGPUColor> for Color {
    fn from(value: WGPUColor) -> Self {
        Color {
            r: (value.r * 255.0) as u8,
            g: (value.g * 255.0) as u8,
            b: (value.b * 255.0) as u8,
            a: (value.a * 255.0) as u8,
        }
    }
}

impl From<u32> for Color {
    fn from(value: u32) -> Self {
        Color {
            r: ((value >> 16) & 255) as u8,
            g: ((value >> 8) & 255) as u8,
            b: (value & 255) as u8,
            a: 255
        }
    }
}

