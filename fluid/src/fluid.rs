


use crate::config;
use crate::utils;



use macroquad::prelude::*;



use utils::{get_directions, Vector};
use config::Config;



#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Cell {
    Fluid,
    Solid,
    Emitter,
}

impl Cell {
    pub fn stringify(&self) -> &str {
        match self {
            Cell::Fluid   => "Fluid",
            Cell::Solid   => "Solid",
            Cell::Emitter => "Emitter",
        }
    }
}

#[derive(Debug)]
pub struct Fluid {
    pub x:  usize,
    pub y:  usize,

    pub u:  Vec<Vec<f32>>,
    pub v:  Vec<Vec<f32>>,
    pub nu: Vec<Vec<f32>>,
    pub nv: Vec<Vec<f32>>,

    pub element:        Vec<Vec<Cell>>,
    
    pub overrelaxation: f32,
    pub iters:          usize,
    pub delta_t:        f32,

    pub cell_size:      f32,
}

impl Fluid {
    pub fn new(config: &Config) -> Fluid {
        Fluid {
            x:              config.x,
            y:              config.y,
            
            overrelaxation: config.overrelaxation,
            iters:          config.iters,
            delta_t:        config.delta_t,

            cell_size:      config.cell_size,
            
            element:        vec![vec![Cell::Fluid; config.x]; config.y],
            
            u:              vec![vec![0.0; config.x + 1]; config.y],
            v:              vec![vec![0.0; config.x]; config.y + 1],
            nu:             vec![vec![0.0; config.x + 1]; config.y],
            nv:             vec![vec![0.0; config.x]; config.y + 1],
        }
    }

    pub fn inbounds(&self, x: usize, y: usize) -> bool {
        x < self.x && y < self.y
    }

    pub fn assert_bounds(&mut self) {
        for y in 0..self.y {
            for x in 0..self.x {
                if y == 0 || y == self.y-1 || x == 0 || x == self.x-1 {
                    self.element[y][x] = Cell::Solid;
                }
            }
        }
    }

    pub fn assert_emitter(&mut self) {
        for y in 0..self.y {
            for x in 0..self.x {
                let xx = self.x;
                let yx = self.y;
                let mut oo: Oo = Oo::new(x, y, self);

                // if (3..=7).contains(&y) && x == 0 {
                //     oo.place(Cell::Emitter);
                //     oo.set_velocity(-1, 0, 30.0);
                //     oo.set_velocity(1, 0, 30.0);
                // }
                if ((yx * 2 / 8)..=(yx * 7 / 8)).contains(&y) && x == 0 {
                    oo.place(Cell::Emitter);
                    oo.set_velocity(-1, 0, 50.0);
                    oo.set_velocity(1, 0, 50.0);
                }
                // if ((yx * 2 / 8)..=(yx * 7 / 8)).contains(&y) && x == xx-1 {
                //     oo.place(Cell::Emitter);
                //     oo.set_velocity(-1, 0, 50.0);
                //     oo.set_velocity(1, 0, 50.0);
                // }

                // if (x == 0 || x == xx-1) && yx * 5 / 16 < y && y < yx * 7 / 16 {
                //     oo.place(Cell::Emitter);
                //     oo.set_velocity(-1, 0, 50.0);
                //     oo.set_velocity(1, 0, 50.0);
                // }

                // if ((xx * 1 / 5)..(xx * 2 / 5)).contains(&x) && ((yx * 2 / 5)..(yx * 3 / 5)).contains(&y) {
                //     oo.place(Cell::Solid);
                // }

                // let center_x = xx / 6;
                // let center_y = yx / 2;
                // let radius = (xx / 30) as f32;

                // let distance = ((y - center_y) * (y - center_y) + (x - center_x) * (x - center_x)) as f32;
                // let distance = distance.sqrt();

                // if distance < radius {
                //     oo.place(Cell::Solid);
                // }

                // if yx * 4 < y && y < yx * 3 / 4 && x == xx-1 {
                //     oo.place(Cell::Emitter);
                //     oo.set_velocity(1, 0, 50.0);
                //     oo.set_velocity(-1, 0, 50.0);
                // }
                // if yx * 5 / 8 < y && y < yx * 3 / 4 && x == xx - 10 {
                //     oo.place(Cell::Solid);
                // }
                // if yx / 4 < y && y < yx / 2 && xx / 14 < x && x < xx / 3 {
                //     oo.place(Cell::Solid);
                // }

                // if xx / 10 < x && x < xx / 5 && y == 0 {
                //     oo.place(Cell::Emitter);
                //     oo.set_velocity(0, -1, 10.0);
                //     oo.set_velocity(0, 1, 10.0);
                // }
                // if xx / 10 < x && x < xx / 5 && y == yx-1 {
                //     oo.place(Cell::Emitter);
                //     oo.set_velocity(0, -1, -10.0);
                //     oo.set_velocity(0, 1, -10.0);
                // }
            }
        }
    }

    pub fn update_fluid(&mut self) {
        self.incompressibility();
        self.advection();
        self.assert_bounds();
        self.assert_emitter();
    }

    pub fn incompressibility(&mut self) {
        for _ in 0..self.iters {

            for y in 0..self.y {
                for x in 0..self.x {

                    if self.element[y][x] != Cell::Fluid {
                        continue;
                    }

                    let mut oo: Oo = Oo::new(x, y, self);

                    let divergence: f32 = oo.divergence();
                    let sides: f32 = oo.sides();

                    if sides == 0.0 {
                        continue;
                    }
                    
                    let correction: f32 = -divergence / sides * oo.fluid.overrelaxation;

                    oo.correction(correction);
                }
            }
        }
    }

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
    
    pub fn print_cli(&self) {
        for y in 0..self.y {
            let mut line: String = String::new();
            for x in 0..self.x {
                line.push_str( self.element[y][x].stringify() );
                line.push(' ');
            }
            println!("{}", line);
        }
        for y in 0..self.y {
            let mut line: String = String::new();
            for x in 0..self.x {
                let velocity: f32 = self.u[y][x] * self.u[y][x] + self.v[y][x] * self.v[y][x];
                let velocity: f32 = velocity.sqrt();

                line.push_str( &velocity.round().to_string() );
                line.push(' ');
            }
            println!("{}", line);
        }
    }

    pub fn draw(&mut self, head: bool) {
        for y in 0..self.y {
            for x in 0..self.x {
                let color: Color = match self.element[y][x] {
                    Cell::Fluid   => continue,
                    Cell::Solid   => Color::from_hex(0xae5a41),
                    Cell::Emitter => Color::from_hex(0x1b85b8),
                };

                draw_rectangle(
                    x as f32 * self.cell_size,
                    y as f32 * self.cell_size,
                    self.cell_size,
                    self.cell_size,
                    color,
                );
            }
        }
        for y in 0..self.y {
            for x in 0..self.x {

                if self.element[y][x] != Cell::Fluid { continue; }
                
                let oo: Oo = Oo::new(x, y, self);
                let mut velocity = Vector::new();
                velocity.add(oo.peek_velocity(1, 0), 0.0);
                velocity.add(oo.peek_velocity(-1, 0), 0.0);
                velocity.add(0.0, oo.peek_velocity(0, 1));
                velocity.add(0.0, oo.peek_velocity(0, -1));               

                velocity.normalize();
                velocity.x *= 9.0;
                velocity.y *= 9.0;

                let start = Vector { 
                    x: x as f32 * self.cell_size + self.cell_size / 2.0, 
                    y: y as f32 * self.cell_size + self.cell_size / 2.0, 
                };

                draw_line(
                    start.x,
                    start.y,
                    start.x + velocity.x,
                    start.y + velocity.y,
                    0.8,
                    Color::from_hex(0xc3cb71),
                );

                if head {
                    draw_circle(
                        start.x + velocity.x, 
                        start.y + velocity.y, 
                        1.2, 
                        Color::from_hex(0xc3cb71),
                    );
                }
            }
        }
    }
}

pub struct Oo<'a> {
    pub x:     usize,
    pub y:     usize,
    pub fluid: &'a mut Fluid,
}

impl<'a> Oo<'a> {
    pub fn new(x: usize, y: usize, fluid: &'a mut Fluid) -> Oo {
        Oo { 
            x, 
            y, 
            fluid 
        }
    }

    pub fn place(&mut self, cell: Cell) {
        self.fluid.element[self.y][self.x] = cell;
    }
    
    pub fn peek(&self, dx: isize, dy: isize) -> Cell {
        let (nx, ny) = self.index(dx, dy);
        if self.fluid.inbounds(nx, ny) {
            self.fluid.element[ny][nx]
        } else {
            Cell::Solid
        }
    }

    pub fn peek_velocity(&self, dx: isize, dy: isize) -> f32 {
        let (nx, ny) = self.index(dx, dy);
        match (dx, dy) {
            (1, 0)  => self.fluid.u[self.y][nx    ],
            (-1, 0) => self.fluid.u[self.y][self.x],
            (0, 1)  => self.fluid.v[ny    ][self.x],
            (0, -1) => self.fluid.v[self.y][self.x],
            _       => {
                eprintln!("OOB Error checking neighbor");
                std::process::exit(1);
            }
        }
    }

    pub fn avg_u(&self) -> f32 {
        let left = self.peek_velocity(-1, 0);
        let right = self.peek_velocity(1, 0);
        (left + right) / 2.0
    }

    pub fn avg_v(&self) -> f32 {
        let up = self.peek_velocity(0, 1);
        let down = self.peek_velocity(0, -1);
        (up + down) / 2.0
    }

    pub fn divergence(&self) -> f32 {
        let mut divergence: f32 = 0.0;
        for (dx, dy) in get_directions() {
            let div = match (dx, dy) {
                (1, 0)  =>  self.peek_velocity(dx, dy),
                (-1, 0) => -self.peek_velocity(dx, dy),
                (0, 1)  =>  self.peek_velocity(dx, dy),
                (0, -1) => -self.peek_velocity(dx, dy),
                _       => {
                    eprintln!("OOB error calculating divergence");
                    std::process::exit(2);
                }
            };
            divergence += div;
        }
        divergence
    }
    
    pub fn correction(&mut self, correction: f32) {
        for (dx, dy) in get_directions() {
            if self.peek(dx, dy) == Cell::Fluid {
                match (dx, dy) {
                    (1, 0)  => self.modify_velocity(dx, dy,  correction),
                    (-1, 0) => self.modify_velocity(dx, dy, -correction),
                    (0, 1)  => self.modify_velocity(dx, dy,  correction),
                    (0, -1) => self.modify_velocity(dx, dy, -correction),
                    _       => {
                        eprintln!("OOB Error applying correction");
                        std::process::exit(4);
                    }
                }
            }
        }
    }

    pub fn modify_velocity(&mut self, dx: isize, dy: isize, modifier: f32) {
        let (nx, ny) = self.index(dx, dy);
        match (dx, dy) {
            (1, 0)  => self.fluid.u[self.y][nx    ] += modifier,
            (-1, 0) => self.fluid.u[self.y][self.x] += modifier,
            (0, 1)  => self.fluid.v[ny    ][self.x] += modifier,
            (0, -1) => self.fluid.v[self.y][self.x] += modifier,
            _       => {
                eprintln!("OOB Error modifying velocity");
                std::process::exit(3);
            }
        }
    }

    pub fn set_velocity(&mut self, dx: isize, dy: isize, set: f32) {
        let (nx, ny) = self.index(dx, dy);
        match (dx, dy) {
            (1, 0)  => self.fluid.u[self.y][nx    ] = set,
            (-1, 0) => self.fluid.u[self.y][self.x] = set,
            (0, 1)  => self.fluid.v[ny    ][self.x] = set,
            (0, -1) => self.fluid.v[self.y][self.x] = set,
            _       => {
                eprintln!("OOB Error setting velocity");
                std::process::exit(6);
            }
        }
    }

    pub fn sides(&self) -> f32 {
        let mut sides: f32 = 0.0;
        for (dx, dy) in get_directions() {
            if self.peek(dx, dy) == Cell::Fluid {
                sides += 1.0;
            }
        }
        sides
    }

    fn index(&self, dx: isize, dy: isize) -> (usize, usize) {
        let nx: usize = ((self.x as isize) + dx) as usize;
        let ny: usize = ((self.y as isize) + dy) as usize;
        (nx, ny)
    }
}
