


use macroquad::prelude::*;



pub fn get_directions() -> [(isize, isize); 4] {
    [
        (-1, 0), 
        (1, 0), 
        (0, -1), 
        (0, 1)
    ]
}

pub struct Vector<T> {
    pub x: T,
    pub y: T,
}

#[allow(dead_code)]
impl Vector<f32> {
    pub fn new() -> Vector<f32> {
        Vector { x: 0.0, y: 0.0 }
    }

    pub fn construct(x: f32, y: f32) -> Vector<f32> {
        Vector { x, y }
    }

    pub fn add(&mut self, x: f32, y: f32) {
        self.x += x;
        self.y += y;
    }

    pub fn normalize(&mut self) {
        let mag: f32 = self.magnitude();
        self.x /= mag;
        self.y /= mag;
    }

    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

pub fn get_color_vec(vec: &Vector<f32>, max: f32) -> Color {
    let mag = vec.magnitude();
    let clamped = mag.clamp(0.0, max);
    let normalized_mag = clamped / max;

    let r = (normalized_mag * 255.0) as u8;
    let g = 100;
    let b = ((1.0 - normalized_mag) * 255.0) as u8;

    Color::from_rgba(r, g, b, 200)
}



/*

    pub fn advection(&mut self) {
        // time step reallocated from config->fluid
        let dt = self.delta_t;

        for y in 0..self.y {
            for x in 0..self.x {

                if self.element[y][x] != Cell::Fluid { continue; }

                // advection of u
                let oo: Oo = Oo::new(x, y, self);
                let curr_pos = Vector::from(
                    x as f32,
                    y as f32 + 0.5,
                );
                let vel = Vector::from(
                    oo.fluid.u[y][x],
                    oo.avg_v(),
                );
                let source = Vector::from(
                    curr_pos.x - vel.x * dt,
                    curr_pos.y - vel.y * dt,
                );

                let interp_u = self.interpolate_staggered(source, "u");
                self.nu[y][x] = interp_u;

                // advection of v
                let oo: Oo = Oo::new(x, y, self);
                let curr_pos = Vector::from(
                    x as f32,
                    y as f32 + 0.5,
                );
                let vel = Vector::from(
                    oo.fluid.v[y][x],
                    oo.avg_u(),
                );
                let source = Vector::from(
                    curr_pos.x - vel.x * dt,
                    curr_pos.y - vel.y * dt,
                );

                let interp_v = self.interpolate_staggered(source, "v");
                self.nv[y][x] = interp_v;
            }
        }

        self.u.clone_from(&self.nu);
        self.v.clone_from(&self.nv);
    }

    pub fn interpolate_staggered(&self, source: Vector<f32>, field: &str) -> f32 {
        let x0 = source.x.floor() as usize;
        let y0 = source.y.floor() as usize;
        let x1 = x0 + 1;
        let y1 = y0 + 1;
        let tx = source.x - x0 as f32;
        let ty = source.y - y0 as f32;

        let (x0, x1, y0, y1) = match field {
            "u" => {
                (
                    x0.clamp(0, self.x),
                    x1.clamp(0, self.x),
                    y0.clamp(0, self.y-1),
                    y1.clamp(0, self.y-1),
                )
            }
            "v" => {
                (
                    x0.clamp(0, self.x-1),
                    x1.clamp(0, self.x-1),
                    y0.clamp(0, self.y),
                    y1.clamp(0, self.y),
                )
            }
            _ => {
                eprintln!("Incorrect field token found");
                std::process::exit(11);
            }
        };

        match field {
            "u" => {
                let v00 = self.u[y0][x0];
                let v01 = self.u[y1][x0];
                let v10 = self.u[y0][x1];
                let v11 = self.u[y1][x1];

                let v0 = (1.0 - tx) * v00 + tx * v10;
                let v1 = (1.0 - tx) * v01 + tx * v11;

                (1.0 - ty) * v0 + ty * v1
            }
            "v" => {
                let v00 = self.v[y0][x0];
                let v01 = self.v[y1][x0];
                let v10 = self.v[y0][x1];
                let v11 = self.v[y1][x1];

                let v0 = (1.0 - tx) * v00 + tx * v10;
                let v1 = (1.0 - tx) * v01 + tx * v11;

                (1.0 - ty) * v0 + ty * v1
            }
            _ => {
                eprintln!("Incorrect field token found");
                std::process::exit(11);
            }
        }
    }

    pub fn iterator(&mut self) -> Iterator::Self {
        for y in 0..self.y {
            for x in 0..self.x {
                return iter;
            }
        }
    }

    pub fn do_something(&mut self) {
        for cell in self.iterator() {
            // do something
        }
    }

    pub fn is_static(&self) -> bool {
        *self == Self::Static
    }

    pub fn is_source(&self) -> bool {
        *self == Self::Source
    }

    pub fn is_match(&self) -> bool {
        *self == Self::Match
    }

    pub fn set_velocity_matched(&mut self, dx: isize, dy: isize) {
        match (dx, dy) {
            (1, 0) => {
                self.fluid.u[self.y][self.x+1] = self.fluid.u[self.y][self.x];
            }
            (-1, 0) => {
                self.fluid.u[self.y][self.x] = self.fluid.u[self.y][self.x-1];
            }
            (0, 1) => {
                self.fluid.v[self.y+1][self.x] = self.fluid.v[self.y][self.x];
            }
            (0, -1) => {
                self.fluid.v[self.y][self.x] = self.fluid.v[self.y-1][self.x];
            }
            _ => {
                eprintln!("Out of accepted matching");
                std::process::exit(8);
            }
        }
    }

*/
