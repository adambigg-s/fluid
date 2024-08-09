


use crate::config;
use crate::utils;
use crate::clone;
use crate::source;
use crate::fluidapi;



use macroquad::prelude::*;



use config::Config;
use utils::{get_color_vec, Vector};
use clone::Clone;
use source::Source;
use fluidapi::Oo;



#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(dead_code)]
pub enum DiffEle {
    Fluid,
    Static,
    Source(Source),
    Clone(Clone),
}


impl DiffEle {
    #[allow(dead_code)]
    pub fn to_strslice(self) -> &'static str {
        match self {
            Self::Fluid     => "Fluid",
            Self::Static    => "Solid",
            Self::Source(_) => "Emitter",
            Self::Clone(_)  => "Match",
        }
    }

    pub fn is_fluid(&self) -> bool {
        matches!(*self, Self::Fluid | Self::Clone(_))
    }

    pub fn is_static(&self) -> bool {
        *self == Self::Static
    }
}

#[derive(Debug)]
pub struct Fluid {
    pub x: usize,
    pub y: usize,

    pub u: Vec<Vec<f32>>,
    pub v: Vec<Vec<f32>>,
    pub nu: Vec<Vec<f32>>,
    pub nv: Vec<Vec<f32>>,
    pub vorticity: Vec<Vec<f32>>,

    pub element: Vec<Vec<DiffEle>>,

    pub overrelaxation: f32,
    pub iters: usize,
    pub delta_t: f32,
    pub source_velocity: f32,
    pub grid_size: f32,
    pub epsilon: f32,

    pub visual_modifier: f32,
    pub cell_size: f32,

    pub boundaries: Vec<Vector<usize>>,
}

impl Fluid {
    pub fn construct(config: &Config) -> Fluid {
        Fluid {
            x: config.x,
            y: config.y,

            u: vec![vec![0.0; config.x + 1]; config.y],
            v: vec![vec![0.0; config.x]; config.y + 1],
            nu: vec![vec![0.0; config.x + 1]; config.y],
            nv: vec![vec![0.0; config.x]; config.y + 1],
            vorticity: vec![vec![0.0; config.x]; config.y],

            element: vec![vec![DiffEle::Fluid; config.x]; config.y],

            overrelaxation: config.overrelaxation,
            iters: config.iters,
            delta_t: config.delta_t,
            source_velocity: config.source_velocity,
            grid_size: config.grid_size,
            epsilon: config.epsilon,

            visual_modifier: config.visual_modifier,
            cell_size: config.cell_size,

            boundaries: Vec::new(),
        }
    }

    pub fn reset(&mut self) {
        self.u = vec![vec![0.0; self.x + 1]; self.y];
        self.v = vec![vec![0.0; self.x]; self.y + 1];
        self.nu = vec![vec![0.0; self.x + 1]; self.y];
        self.nv = vec![vec![0.0; self.x]; self.y + 1];
        self.element = vec![vec![DiffEle::Fluid; self.x]; self.y];
        self.boundaries = Vec::new();
        self.assert_boundary_conditions();
    }

    pub fn inbounds(&self, x: usize, y: usize) -> bool {
        x < self.x && y < self.y
    }

    pub fn assert_boundary_place(&mut self, x: usize, y: usize) {
        if self.inbounds(x, y) {
            let mut oo: Oo = Oo::construct(x, y, self);
            oo.set_here(DiffEle::Static);
        }
    }

    pub fn assert_boundary_delete(&mut self, x: usize, y: usize) {
        if self.inbounds(x, y) {
            let mut oo: Oo = Oo::construct(x, y, self);
            oo.remove_here();
        }
    }

    pub fn assert_boundary_conditions(&mut self) {
        for y in 0..self.y {
            for x in 0..self.x {
                
                let xx: usize = self.x;
                let yy: usize = self.y;
                let mut oo: Oo = Oo::construct(x, y, self);
                
                // if y == 0 || y == yy - 1  {
                //     oo.set_here(DiffEle::Static);
                //     oo.set_velocity_zeros();
                // }

                if x == 0 {
                    let source = Source::construct(oo.fluid.source_velocity, 0.0);
                    oo.set_here(DiffEle::Source(source));
                }

                if x == xx-1 {
                    let cloned = Clone::construct(-1, 0);
                    oo.set_here(DiffEle::Clone(cloned));
                }

                if y == 0 {
                    let cloned = Clone::construct(0, 1);
                    oo.set_here(DiffEle::Clone(cloned));
                }

                if y == yy-1 {
                    let cloned = Clone::construct(0, -1);
                    oo.set_here(DiffEle::Clone(cloned));
                }
                // if y == yy-1 {
                //     oo.set_here(DiffEle::Static);
                // }

                let center_x = xx / 10;
                let center_y = yy / 2;
                let radius = (std::cmp::min(xx, yy) as f32).sqrt() - 3.0;
                if (x as f32 - center_x as f32).powf(2.0) + (y as f32 - center_y as f32).powf(2.0) 
                    < radius * radius 
                {
                    oo.set_here(DiffEle::Static);
                    oo.set_velocity_zeros();
                }

                // if xx / 10 < x && x < xx * 2 / 10 && yy * 2 / 5 < y && y < yy * 3 / 5 {
                //     oo.set_here(DiffEle::Static);
                // }
                // if xx * 3 / 10 < x && x < xx * 4 / 10 && yy * 4 / 5 < y && y < yy {
                //     oo.set_here(DiffEle::Static);
                // }
            }
        }
        self.enforce_boundary_conditions();
    }

    #[allow(dead_code)]
    pub fn print_cli(&self) {
        for y in 0..self.y {
            let mut line: String = String::new();
            for x in 0..self.x {
                line.push_str(self.element[y][x].to_strslice());
                line.push(' ');
            }
            println!("{}", line);
        }
        for y in 0..self.y {
            let mut line: String = String::new();
            for x in 0..self.x {
                let velocity: f32 = self.u[y][x] * self.u[y][x] + self.v[y][x] * self.v[y][x];
                let velocity: f32 = velocity.sqrt();

                line.push_str(&velocity.round().to_string());
                line.push(' ');
            }
            println!("{}", line);
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn display(
        &mut self,
        head: bool,
        draw_bounds: bool,
        normalize: bool,
        thickness: f32,
        head_size: f32,
        spacing: usize,
        vector: bool,
        fill: bool,
    ) {
        for y in 0..self.y {
            for x in 0..self.x {
                let color: Color = match self.element[y][x] {
                    DiffEle::Fluid     => continue,
                    DiffEle::Static    => Color::from_hex(0x000000),
                    DiffEle::Source(_) => Color::from_hex(0x1b85b8),
                    DiffEle::Clone(_)  => Color::from_hex(0x559e83),
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
                if !draw_bounds && self.element[y][x] != DiffEle::Fluid {
                    continue;
                }
                if x % spacing != 0 || y % spacing != 0 {
                    continue;
                }

                let oo: Oo = Oo::construct(x, y, self);
                let mut velocity = Vector::new();
                velocity.add(oo.peek_velocity(1, 0), 0.0);
                velocity.add(oo.peek_velocity(-1, 0), 0.0);
                velocity.add(0.0, oo.peek_velocity(0, 1));
                velocity.add(0.0, oo.peek_velocity(0, -1));

                let color: Color = get_color_vec(
                    &velocity, 
                    oo.fluid.source_velocity, 
                    oo.fluid.visual_modifier
                );

                if vector {
                    let nsize = self.cell_size * 2.5;
                    if normalize {
                        velocity.normalize();
                        velocity.x *= nsize;
                        velocity.y *= nsize;
                    }

                    let start: Vector<f32> = Vector {
                        x: x as f32 * self.cell_size + self.cell_size / 2.0,
                        y: y as f32 * self.cell_size + self.cell_size / 2.0,
                    };
                    draw_line(
                        start.x,
                        start.y,
                        start.x + velocity.x,
                        start.y + velocity.y,
                        thickness,
                        color,
                    );
                    
                    if head {
                        draw_circle(start.x + velocity.x, start.y + velocity.y, head_size, color);
                    }
                } 
                else if fill {
                    let start: Vector<f32> = Vector {
                        x: x as f32 * self.cell_size,
                        y: y as f32 * self.cell_size,
                    };
                    draw_rectangle(start.x, start.y, self.cell_size, self.cell_size, color);
                }
            }
        }
    }

    pub fn update_fluid(&mut self, project: bool, advect: bool, enforce_bc: bool, vort_confinement: bool) {
        if advect {
            self.semi_lagrangian_advection();
        }
        if vort_confinement {
            self.apply_vorticity_confinement();
        }
        if enforce_bc {
            self.enforce_boundary_conditions();
        }
        if project {
            self.projection_gauss_seidel();
        }
    }

    fn projection_gauss_seidel(&mut self) {
        for _ in 0..self.iters {
            
            for y in 0..self.y {
                for x in 0..self.x {
                    
                    if self.element[y][x] != DiffEle::Fluid {
                        continue;
                    }

                    let mut oo: Oo = Oo::construct(x, y, self);

                    let divergence: f32 = oo.divergence_here();
                    let sides: f32 = oo.afflicted_area();

                    if sides == 0.0 {
                        continue;
                    }

                    let correction: f32 = -divergence / sides * oo.fluid.overrelaxation;
                    oo.modify_adjacent(correction);
                }
            }
        }
    }

    fn semi_lagrangian_advection(&mut self) {
        let dt: f32 = self.delta_t;
        let size: f32 = self.grid_size;

        // iterates though cells sans border
        for i in 1..self.y {
            for j in 1..self.x {

                // u component 
                if i < self.y-1 && !self.element[i][j-1].is_static() && !self.element[i][j].is_static() {
                    let u = self.u[i][j];
                    let v = self.average_v(j, i);

                    let mut x = j as f32;
                    let mut y = i as f32 + 0.5;

                    x -= u * dt / size;
                    y -= v * dt / size;

                    self.nu[i][j] = self.double_lin_int(x, y, "u");
                }

                // v component
                if j < self.x-1 && !self.element[i-1][j].is_static() && !self.element[i][j].is_static() {
                    let u = self.average_u(j, i);
                    let v = self.v[i][j];

                    let mut x = j as f32 + 0.5;
                    let mut y = i as f32;

                    x -= u * dt / size;
                    y -= v * dt / size;

                    self.nv[i][j] = self.double_lin_int(x, y, "v");
                }
            }
        }

        self.u.clone_from(&self.nu);
        self.v.clone_from(&self.nv);
    }

    fn double_lin_int(&self, x: f32, y: f32, field: &str) -> f32 {
        let (field, dx, dy): (&Vec<Vec<f32>>, f32, f32) = match field {
            "u" => (&self.u, 0.0, 0.5),
            "v" => (&self.v, 0.5, 0.0),
            _   => {
                eprintln!("Error in field token");
                std::process::exit(99);
            }
        };

        let x = (x - dx).clamp(0.0, (self.x-1) as f32);
        let y = (y - dy).clamp(0.0, (self.y-1) as f32);

        let x0 = x.floor() as usize;
        let y0 = y.floor() as usize;
        let x1 = (x0 + 1).min(self.x-1);
        let y1 = (y0 + 1).min(self.y-1);

        let tx = x - x0 as f32; 
        let ty = y - y0 as f32;

        let sx = 1.0 - tx;
        let sy = 1.0 - ty;

        sx * sy * field[y0][x0]
            + tx * sy * field[y0][x1]
            + tx * ty * field[y1][x1]
            + sx * ty * field[y1][x0]
    }

    fn average_u(&self, x: usize, y: usize) -> f32 {
        (self.u[y-1][x] + self.u[y-1][x+1] + self.u[y][x] + self.u[y][x+1]) * 0.25
    }

    fn average_v(&self, x: usize, y: usize) -> f32 {
        (self.v[y+1][x-1] + self.v[y+1][x] + self.v[y][x-1] + self.v[y][x]) * 0.25
    }

    fn apply_vorticity_confinement(&mut self) {
        self.compute_vorticity();
        let mut force_x = vec![vec![0.0; self.x]; self.y];
        let mut force_y = vec![vec![0.0; self.x]; self.y];

        for i in 1..self.y-1 {
            for j in 1..self.x-1 {

                let grad_w_x = (self.vorticity[i][j+1] - self.vorticity[i][j-1]) * 0.5;
                let grad_w_y = (self.vorticity[i+1][j] - self.vorticity[i-1][j]) * 0.5;

                let magnitude = Vector::construct(grad_w_x, grad_w_y).magnitude();
                if magnitude > 1e-6 {
                    let nx = grad_w_x / magnitude;
                    let ny = grad_w_y / magnitude;

                    force_x[i][j] = self.epsilon * (ny * self.vorticity[i][j]);
                    force_y[i][j] = -self.epsilon * (nx * self.vorticity[i][j]);
                }
            }
        }

        for i in 1..self.y-1 {
            for j in 1..self.x-1 {
                self.u[i][j] += force_x[i][j] * self.delta_t;
                self.v[i][j] += force_y[i][j] * self.delta_t;
            }
        }
    }
    
    fn compute_vorticity(&mut self) {
        for i in 1..self.y-1 {
            for j in 1..self.x-1 {

                let dwdy = (self.u[i+1][j] - self.u[i-1][j]) * 0.5;
                let dudx = (self.v[i][j+1] - self.v[i][j-1]) * 0.5;

                self.vorticity[i][j] = dwdy - dudx;
            }
        }
    }

    fn enforce_boundary_conditions(&mut self) {
        for position in self.boundaries.clone() {
            let mut oo: Oo = Oo::construct(position.x, position.y, self);
            match oo.peek_element_here(0, 0) {
                DiffEle::Static       => {
                    oo.set_velocity_zeros();
                }
                DiffEle::Source(sour) => {
                    oo.set_velocity_polarized(sour.velocity.x, sour.velocity.y);
                }
                DiffEle::Clone(clo)   => {
                    oo.set_velocity_matched(clo.master.x, clo.master.y);
                }
                _                     => {}
            }
        }
    }
}

