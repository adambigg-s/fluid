


use crate::config;
use crate::utils;



use macroquad::prelude::*;



use config::Config;
use utils::{get_color_vec, get_directions, Vector};



#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(dead_code)]
pub enum DiffEle {
    Fluid,
    Static,
    Source,
    Match(Match),
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Match {
    x: usize,
    y: usize,
    master: Vector<isize>,
}

impl Match {
    pub fn construct(x: usize, y: usize, mx: isize, my: isize) -> Match {
        let master: Vector<isize> = Vector::construct(mx, my);
        Match { x, y, master }
    }
}

impl DiffEle {
    pub fn to_strslice(self) -> &'static str {
        match self {
            DiffEle::Fluid    => "Fluid",
            DiffEle::Static   => "Solid",
            DiffEle::Source   => "Emitter",
            DiffEle::Match(_) => "Match",
        }
    }

    pub fn is_fluid(&self) -> bool {
        matches!(*self, Self::Fluid | Self::Match(_))
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

    pub element: Vec<Vec<DiffEle>>,

    pub overrelaxation: f32,
    pub iters: usize,
    pub delta_t: f32,

    pub cell_size: f32,

    pub matches: Vec<Match>,
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

            element: vec![vec![DiffEle::Fluid; config.x]; config.y],

            overrelaxation: config.overrelaxation,
            iters: config.iters,
            delta_t: config.delta_t,

            cell_size: config.cell_size,

            matches: Vec::new(),
        }
    }

    pub fn reset(&mut self) {
        self.u = vec![vec![0.0; self.x + 1]; self.y];
        self.v = vec![vec![0.0; self.x]; self.y + 1];
        self.nu = vec![vec![0.0; self.x + 1]; self.y];
        self.nv = vec![vec![0.0; self.x]; self.y + 1];
        self.element = vec![vec![DiffEle::Fluid; self.x]; self.y];
        self.matches = Vec::new();
        self.assert_boundary_conditions();
    }

    pub fn inbounds(&self, x: usize, y: usize) -> bool {
        x < self.x && y < self.y
    }

    pub fn assert_boundary_place(&mut self, x: usize, y: usize) {
        if self.inbounds(x, y) {
            self.element[y][x] = DiffEle::Static;
        }
    }

    pub fn assert_match_velocity(&mut self) {
        for mat in self.matches.clone() {
            let mut oo: Oo = Oo::construct(mat.x, mat.y, self);
            oo.set_velocity_matched(mat.master.x, mat.master.y);
        }
    }

    pub fn assert_boundary_conditions(&mut self) {
        for y in 0..self.y {
            for x in 0..self.x {
                let xx: usize = self.x;
                let yy: usize = self.y;
                let mut oo: Oo = Oo::construct(x, y, self);
                if oo.fluid.element[y][x].is_static() {
                    oo.set_velocity_zeros();
                }

                // if y == 0 || y == yy - 1 || x == 0 {
                //     oo.set_here(DiffEle::Static);
                //     oo.set_velocity_zeros();
                // }

                if x == 0 {
                    oo.set_here(DiffEle::Source);
                    oo.set_velocity_polarized(60.0, 0.0);
                }
                // if yy * 2 / 5 < y && y < yy * 3 / 5 && x == 0 {
                //     oo.set_here(DiffEle::Source);
                //     oo.set_velocity_polarized(60.0, 0.0);
                // }
                // if x == xx-1 {
                //     oo.set_here(DiffEle::Source);
                //     oo.set_velocity_polarized(45.0, 0.0);
                // }

                if x == xx - 1 {
                    let matched = Match::construct(x, y, -1, 0);
                    oo.set_here(DiffEle::Match(matched));
                }
                if y == 0 {
                    let matched = Match::construct(x, y, 0, 1);
                    oo.set_here(DiffEle::Match(matched));
                }
                if y == yy-1 {
                    let matched = Match::construct(x, y, 0, -1);
                    oo.set_here(DiffEle::Match(matched));
                }

                // let center_x = xx / 10;
                // let center_y = yy / 2;
                // let radius = (std::cmp::min(xx, yy) as f32).sqrt() - 3.0;
                // if (x as f32 - center_x as f32).powf(2.0) + (y as f32 - center_y as f32).powf(2.0) 
                //     < radius * radius 
                // {
                //     oo.set_here(DiffEle::Static);
                //     oo.set_velocity_zeros();
                // }

                // if xx / 10 < x && x < xx * 2 / 10 && yy * 2 / 5 < y && y < yy * 3 / 5 {
                //     oo.set_here(DiffEle::Static);
                // }
            }
        }
    }

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
                    DiffEle::Fluid    => continue,
                    DiffEle::Static   => Color::from_hex(0x000000),
                    DiffEle::Source   => Color::from_hex(0x1b85b8),
                    DiffEle::Match(_) => Color::from_hex(0x559e83),
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

                let color: Color = get_color_vec(&velocity, 200.0);

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
                } else if fill {
                    let start: Vector<f32> = Vector {
                        x: x as f32 * self.cell_size,
                        y: y as f32 * self.cell_size,
                    };
                    draw_rectangle(start.x, start.y, self.cell_size, self.cell_size, color);
                }
            }
        }
    }

    pub fn update_fluid(&mut self, project: bool, advect: bool, assert_bc: bool) {
        if project {
            self.projection_gauss_seidel();
            self.assert_match_velocity();
        }
        if advect {
            self.semi_lagrangian_advection();
        }
        if assert_bc {
            self.assert_boundary_conditions();
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
        for y in 0..self.y {
            for x in 0..self.x {
                if self.element[y][x] != DiffEle::Fluid {
                    continue;
                }

                let prev_x =
                    (x as f32 - self.u[y][x] * self.delta_t / self.cell_size).round() as usize;
                let prev_y =
                    (y as f32 - self.v[y][x] * self.delta_t / self.cell_size).round() as usize;

                if self.inbounds(prev_x, prev_y) {
                    self.nu[y][x] = self.u[prev_y][prev_x];
                    self.nv[y][x] = self.v[prev_y][prev_x];
                }
            }
        }
        self.u.clone_from(&self.nu);
        self.v.clone_from(&self.nv);
    }
}

pub struct Oo<'a> {
    pub x: usize,
    pub y: usize,
    pub fluid: &'a mut Fluid,
}

impl<'a> Oo<'a> {
    pub fn construct(x: usize, y: usize, fluid: &'a mut Fluid) -> Oo {
        Oo { x, y, fluid }
    }

    pub fn set_here(&mut self, cell: DiffEle) {
        match cell {
            DiffEle::Match(mat) => {
                self.fluid.matches.push(mat);
                self.fluid.element[self.y][self.x] = cell;
            }
            _                   => {
                self.fluid.element[self.y][self.x] = cell;
            }
        }
    }

    pub fn peek_element_here(&self, dx: isize, dy: isize) -> DiffEle {
        let (nx, ny): (usize, usize) = self.index(dx, dy);
        if self.fluid.inbounds(nx, ny) {
            self.fluid.element[ny][nx]
        } else {
            DiffEle::Static
        }
    }

    pub fn peek_velocity(&self, dx: isize, dy: isize) -> f32 {
        let (nx, ny): (usize, usize) = self.index(dx, dy);
        match (dx, dy) {
            (1, 0)  => self.fluid.u[self.y][nx],
            (-1, 0) => self.fluid.u[self.y][self.x],
            (0, 1)  => self.fluid.v[ny][self.x],
            (0, -1) => self.fluid.v[self.y][self.x],
            _       => {
                eprintln!("OOB Error checking neighbor");
                std::process::exit(1);
            }
        }
    }

    pub fn peek_velocity_mut(&mut self, dx: isize, dy: isize) -> &mut f32 {
        let (nx, ny): (usize, usize) = self.index(dx, dy);
        match (dx, dy) {
            (1, 0)  => &mut self.fluid.u[self.y][nx],
            (-1, 0) => &mut self.fluid.u[self.y][self.x],
            (0, 1)  => &mut self.fluid.v[ny][self.x],
            (0, -1) => &mut self.fluid.v[self.y][self.x],
            _       => {
                eprintln!("OOB Error fetching neighbor velocity reference");
                std::process::exit(37);
            }
        }
    }

    pub fn divergence_here(&self) -> f32 {
        self.peek_velocity(1, 0) 
            - self.peek_velocity(-1, 0) 
            + self.peek_velocity(0, 1)
            - self.peek_velocity(0, -1)
    }

    pub fn modify_adjacent(&mut self, adjustment: f32) {
        if self.peek_element_here(1, 0).is_fluid() {
            *self.peek_velocity_mut(1, 0) += adjustment;
        }
        if self.peek_element_here(-1, 0).is_fluid() {
            *self.peek_velocity_mut(-1, 0) += -adjustment;
        }
        if self.peek_element_here(0, 1).is_fluid() {
            *self.peek_velocity_mut(0, 1) += adjustment;
        }
        if self.peek_element_here(0, -1).is_fluid() {
            *self.peek_velocity_mut(0, -1) += -adjustment;
        }
    }

    pub fn set_velocity_polarized(&mut self, set_x: f32, set_y: f32) {
        *self.peek_velocity_mut(1, 0) = set_x;
        *self.peek_velocity_mut(-1, 0) = set_x;
        *self.peek_velocity_mut(0, 1) = set_y;
        *self.peek_velocity_mut(0, -1) = set_y;
    }

    pub fn set_velocity_zeros(&mut self) {
        *self.peek_velocity_mut(1, 0) = 0.0;
        *self.peek_velocity_mut(-1, 0) = 0.0;
        *self.peek_velocity_mut(0, 1) = 0.0;
        *self.peek_velocity_mut(0, -1) = 0.0;
    }

    pub fn set_velocity_matched(&mut self, dref_x: isize, dref_y: isize) {
        let (rx, ry) = self.index(dref_x, dref_y);
        let v10: f32;
        let vn0: f32;
        let v01: f32;
        let v0n: f32;
        let damping = 1.0;
        {
            let refr = Oo::construct(rx, ry, self.fluid);
            v10 = refr.peek_velocity(1, 0);
            vn0 = refr.peek_velocity(-1, 0);
            v01 = refr.peek_velocity(0, 1);
            v0n = refr.peek_velocity(0, -1);
        }
        *self.peek_velocity_mut(1, 0) = v10 * damping;
        *self.peek_velocity_mut(-1, 0) = vn0 * damping;
        *self.peek_velocity_mut(0, 1) = v01 * damping;
        *self.peek_velocity_mut(0, -1) = v0n * damping;
    }

    pub fn afflicted_area(&self) -> f32 {
        let mut sides: f32 = 0.0;
        for (dx, dy) in get_directions() {
            if self.peek_element_here(dx, dy).is_fluid() {
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
