use crate::{Canvas, geometry::Vec2};

pub struct Sprite {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<u8>
}

impl Sprite {
    pub fn new(path: &str) -> Self {
        let image = image::open(path).expect("no file path x_x").to_rgba8();

        let mut pixels: Vec<u8> = Vec::new();
        for pixel in image.pixels() {
            pixels.append(&mut pixel.0.to_vec());
        }

        Self {
            width: image.width(),
            height: image.height(),
            pixels
        }
    }

    pub fn draw(&self, canvas: &mut Canvas, point: &Vec2) {
        for y in 0..self.height {
            for x in 0..self.width {
                let index = 4 * (y * self.width + x) as usize;
                let slice = &self.pixels[index..index + 4];
                if slice[3] == 0 { continue };

                let pixel: [u8; 4] = slice.try_into().unwrap();
                canvas.draw_nearest_pixel(point.x + x as f32, point.y + y as f32, pixel);
            }
        }
    }
}

