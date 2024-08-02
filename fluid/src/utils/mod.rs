use std::ops::MulAssign;




pub fn get_directions() -> [(isize, isize); 4] {
    [
        (-1,  0),
        ( 1,  0),
        ( 0, -1),
        ( 0,  1),
    ]
}

pub struct Vector<T> {
    pub x: T,
    pub y: T,
}

impl Vector<f32> {
    pub fn new() -> Vector<f32> {
        Vector {
            x: 0.0,
            y: 0.0,
        }
    }

    pub fn from(x: f32, y: f32) -> Vector<f32> {
        Vector {
            x,
            y,
        }
    }

    pub fn add(&mut self, x: f32, y: f32) {
        self.x += x;
        self.y += y;
    }

    pub fn normalize(&mut self) {
        let mag = self.x * self.x + self.y * self.y;
        let mag_sq = mag.sqrt();
        self.x /= mag_sq;
        self.y /= mag_sq;
    }
}
