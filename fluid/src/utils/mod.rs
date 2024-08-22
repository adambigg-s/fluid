


use crate::fluid;



use macroquad::prelude::*;



use fluid::Fluid;



/// 4 directions adjacent to a cell on a cartesian grid
pub fn get_directions() -> [(isize, isize); 4] {
    [
        (-1, 0), 
        (1, 0), 
        (0, -1), 
        (0, 1)
    ]
}

/// 8 directions adjacent to a cell on a cartesian grid 
/// differs from get_directions() by the addition of corner cells 
/// with no actual direct contact with master cell 
#[allow(dead_code)]
pub fn get_directions_8() -> [(isize, isize); 8] {
    [
        (-1, 0), 
        (1, 0), 
        (0, -1), 
        (0, 1),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ]
}

/// returns all adjacent cell's relative indicies in a 3d cartesian
/// grid including corner cells with no direct flux into master
#[allow(dead_code)]
pub fn get_directions_26() -> [(isize, isize, isize); 26] {
    [
        (1 , 0 , 0), (1 , 1 , 0) , 
        (1 , -1, 0), (0 , 1 , 0) , 
        (0 , -1, 0), (-1, 0 , 0) , 
        (-1, 1 , 0), (-1, -1, 0) , 
        (0 , 0 , 1), (0 , 0 , -1),
        (1 , 0 , 1), (1 , 0 , -1), 
        (1 , 1 , 1), (1 , 1 , -1), 
        (1 , -1, 1), (1 , -1, -1), 
        (-1, 0 , 1), (-1, 0 , -1), 
        (-1, 1 , 1), (-1, 1 , -1),
        (-1, -1, 1), (-1, -1, -1), 
        (0 , 1 , 1), (0 , 1 , -1), 
        (0 , -1, 1), (0 , -1, -1),
    ]
}

fn iter_grid(x: usize, y: usize, bound: isize) -> impl Iterator<Item = (usize, usize)> {
    (-bound..=bound).flat_map(move |dx| {
        (-bound..=bound).map(move |dy| {
            ((x as isize + dx) as usize, (y as isize + dy) as usize)
        })
    })
}

#[allow(dead_code)]
pub trait Clamp {
    fn clamped(&self, min: Self, max: Self) -> Self;
}

impl Clamp for f32 {
    #[cfg(target_arch = "x86_64")]
    fn clamped(&self, min: f32, max: f32) -> f32 {
        let value: f32 = *self;
        let result: f32;

        unsafe {
            std::arch::asm!(
                "movss xmm0, {value}",
                "movss xmm1, {min}",
                "movss xmm2, {max}",
                
                "maxss xmm0, xmm1",
                
                "minss xmm0, xmm2",
                
                "movss {result}, xmm0",

                value  = in(xmm_reg)  value,
                min    = in(xmm_reg)  min,
                max    = in(xmm_reg)  max,
                result = out(xmm_reg) result,
                options(nostack),
            );
        }

        result
    }
    
    #[cfg(not(target_arch = "x86_64"))]
    fn clamped(&self, min: f32, max: f32) -> f32{
        assert!(min <= max);
        if self < &min {
            min
        } else if self > &max {
            max
        } else {
            *self
        }
    }
}

#[derive(PartialEq, Clone, Copy, Debug, Eq, Hash)]
pub struct Vector<T> {
    pub x: T,
    pub y: T,
}

pub trait ToVector {
    fn to_vector(&self) -> Vector<f32>;
}

impl ToVector for (f32, f32) {
    fn to_vector(&self) -> Vector<f32> {
        Vector::construct(self.0, self.1)
    }
}

impl<T: Default> Vector<T> {
    pub fn new() -> Vector<T> {
        Vector {
            x: T::default(),
            y: T::default(),
        }
    }
}

impl<T> Vector<T> {
    pub fn construct(x: T, y: T) -> Vector<T> {
        Vector { x, y }
    }
}

#[allow(dead_code)]
impl Vector<isize> {
    pub fn dot(v1: Self, v2: Self) -> isize {
        v1.x * v2.x + v1.y * v2.y
    }
}

#[allow(dead_code)]
impl Vector<f32> {
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

pub fn get_color_vec(vec:&Vector<f32>, max: f32, buffer_mult: f32) -> Color {
    let max = max * buffer_mult;
    
    let mag = vec.magnitude();
    let clamped = mag.clamp(0.0, max);
    let norm_mag = clamped / max;

    // currently magic number -- scaled_norm_mag adjusts the hue curve, with a higher power giving 
    // more resolution to lower velocities and a lower power (especially < 1.0) giving a much more 
    // dynamic colorization to higher velocitys. eventually this needs to be automated for sim conditions
    let scaled_norm_mag = norm_mag.powf(1.6);

    // color scheme can be inverted easily by changing to <scaled_norm_mag * 360.0>
    let hue = 360.0 - (scaled_norm_mag * 360.0);
    let saturation = 1.0;
    let value = 1.0;
    let (r, g, b) = hsv_to_rgb(hue, saturation, value);

    Color::from_rgba(r, g, b, 200)
}

fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (u8, u8, u8) {
    let c = v * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = v - c;

    let (r, g, b) = match h as u32 {
        0..=59    => (c, x, 0.0),
        60..=119  => (x, c, 0.0),
        120..=179 => (0.0, c, x),
        180..=239 => (0.0, x, c),
        240..=299 => (x, 0.0, c),
        300..=359 => (c, 0.0, x),
        _         => (1.0, 1.0, 1.0),
    };

    (
        ((r + m) * 255.0) as u8,
        ((g + m) * 255.0) as u8,
        ((b + m) * 255.0) as u8,
    )
}

pub fn interpolate_f32(curr: Vector<f32>, prev: Vector<f32>) -> Vec<Vector<f32>> {
    let dx = prev.x - curr.x;
    let dy = prev.y - curr.y;
    let distance = Vector::construct(dx, dy).magnitude();
    let steps = distance as usize;
    let mut points = Vec::new();

    if steps < 1 {
        points.push(prev);
        return points;
    }

    for i in 0..=steps {
        let t = (i as f32) / distance;
        points.push(
            Vector::construct(
                prev.x + t * dx,
                prev.y + t * dy,
            )
        );
    }

    points
}

pub fn place_tool(prev: &mut Option<Vector<f32>>, fluid: &mut Fluid, mode: &str, size: usize) {
    match mode {
        "place" => {
            let now = mouse_position().to_vector();
            if let Some(prev) = prev {
                let points = interpolate_f32(*prev, now);
                for point in points {
                    for (nx, ny) in iter_grid(
                        (point.x / fluid.cell_size) as usize, 
                        (point.y / fluid.cell_size) as usize,
                        size as isize,
                    ) {
                        fluid.assert_boundary_place(nx, ny);
                    }
                }
            }
            *prev = Some(now);
        }
        "delete" => {
            let now = mouse_position().to_vector();
            if let Some(prev) = prev {
                let points = interpolate_f32(*prev, now);
                for point in points {
                    for (nx, ny) in iter_grid(
                        (point.x / fluid.cell_size) as usize, 
                        (point.y / fluid.cell_size) as usize,
                        size as isize,
                    ) {
                        fluid.assert_boundary_delete(nx, ny);
                    }
                }
            }
            *prev = Some(now);
        }
        _ => {}
    }
}

