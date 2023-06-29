// code adapted (taken) from https://docs.rs/line_drawing/latest/src/line_drawing/bresenham.rs.html#27-34

use crate::geometry::Vector;

struct Octant(u8);

impl Octant {
    fn new(start: &Vector, end: &Vector) -> Self {
        let mut value = 0;
        let mut dx = (*end - *start).x;
        let mut dy = (*end - *start).y;

        if dy < 0.0 {
            dx = -dx;
            dy = -dy;
            value += 4;
        }

        if dx < 0.0 {
            let tmp = dx;
            dx = dy;
            dy = -tmp;
            value += 2
        }

        if dx < dy {
            value += 1
        }

        Self(value)
    }

    // return point that's in octant 0
    fn to(&self, point: &Vector) -> Vector {
        match self.0 {
            0 => Vector::new(point.x, point.y),
            1 => Vector::new(point.y, point.x),
            2 => Vector::new(point.y, -point.x),
            3 => Vector::new(-point.x, point.y),
            4 => Vector::new(-point.x, -point.y),
            5 => Vector::new(-point.y, -point.x),
            6 => Vector::new(-point.y, point.x),
            7 => Vector::new(point.x, -point.y),
            _ => unreachable!(),
        }
    }

    // return original point given current octant
    fn from(&self, point: &Vector) -> Vector {
        match self.0 {
            0 => Vector::new(point.x, point.y),
            1 => Vector::new(point.y, point.x),
            2 => Vector::new(-point.y, point.x),
            3 => Vector::new(-point.x, point.y),
            4 => Vector::new(-point.x, -point.y),
            5 => Vector::new(-point.y, -point.x),
            6 => Vector::new(point.y, -point.x),
            7 => Vector::new(point.x, -point.y),
            _ => unreachable!(),
        }
    }
}

pub struct Bresenham {
    octant: Octant,
    point: Vector,
    end_x: f32,
    delta_x: f32,
    delta_y: f32,
    error: f32,
}

impl Bresenham {
    pub fn new(start: &Vector, end: &Vector) -> Self {
        let octant = Octant::new(start, end);
        let start = octant.to(start);
        let end = octant.to(end);

        let delta_x = end.x - start.x;
        let delta_y = end.y - start.y;

        Self {
            delta_x,
            delta_y,
            octant,
            point: start,
            end_x: end.x,
            error: delta_y - delta_x,
        }
    }
}

impl Iterator for Bresenham {
    type Item = Vector;
    fn next(&mut self) -> Option<Self::Item> {
        if self.point.x > self.end_x {
            return None;
        }

        let point = self.octant.from(&self.point);

        if self.error >= 0.0 {
            self.point.y += 1.0;
            self.error -= self.delta_x;
        }

        self.point.x += 1.0;
        self.error += self.delta_y;

        Some(point)
    }
}
