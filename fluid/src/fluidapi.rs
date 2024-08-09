


use crate::fluid;
use crate::utils;



use fluid::{DiffEle, Fluid};
use utils::{get_directions, Vector};



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
        self.fluid.element[self.y][self.x] = cell;
        if !self.fluid.boundaries.contains( &Vector::construct(self.x, self.y) ){
            self.fluid.boundaries.push( Vector::construct(self.x, self.y) );
        }
    }

    pub fn remove_here(&mut self) {
        self.fluid.element[self.y][self.x] = DiffEle::Fluid;
        if let Some(idx) = self.fluid.boundaries.iter().position(
            |vec| *vec == Vector::construct(self.x, self.y)
        ) {
            self.fluid.boundaries.remove(idx);
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
        *self.peek_velocity_mut(1, 0)  = set_x;
        *self.peek_velocity_mut(-1, 0) = set_x;
        *self.peek_velocity_mut(0, 1)  = set_y;
        *self.peek_velocity_mut(0, -1) = set_y;
    }

    pub fn set_velocity_zeros(&mut self) {
        *self.peek_velocity_mut(1, 0)  = 0.0;
        *self.peek_velocity_mut(-1, 0) = 0.0;
        *self.peek_velocity_mut(0, 1)  = 0.0;
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
        *self.peek_velocity_mut(1, 0)  = v10 * damping;
        *self.peek_velocity_mut(-1, 0) = vn0 * damping;
        *self.peek_velocity_mut(0, 1)  = v01 * damping;
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
