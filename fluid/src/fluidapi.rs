


use crate::fluid;
use crate::utils;



use std::arch;



use fluid::{Ele, Fluid};
use utils::{get_directions, Vector};



/// represents a coordinate in the cartesian fluid grid along with a mutable reference to the fluid struct 
///
/// the Oo struct is used to safely interact with the staggered fluid grids a specific (x, y) coordinate
/// and their adjacent cells. this wrapper struct provides methods to modify the fluid and it's properties
/// without having to deal with remembering the conventions for the relative indexing of a staggered grid
/// allocation of velocites
pub struct Oo<'a> {
    /// the x coordinate of the cell in the fluid grid
    pub x: usize,
    
    /// the y coordinate of the cell in the fluid grid
    pub y: usize,

    /// a mutable reference to the Fluid struct allowing modifaction and reading of data by Oo 
    pub fluid: &'a mut Fluid,
}

impl<'a> Oo<'a> {
    pub fn construct(x: usize, y: usize, fluid: &'a mut Fluid) -> Oo {
        Oo { x, y, fluid }
    }

    /// places an Ele at the current Oo's location 
    pub fn set_here(&mut self, cell: Ele) {
        self.fluid.element[self.y][self.x] = cell;
        if !self.fluid.boundaries.contains( &Vector::construct(self.x, self.y) ){
            self.fluid.boundaries.push( Vector::construct(self.x, self.y) );
        }
    }

    /// reverts the current cell back to Fluid
    pub fn remove_here(&mut self) {
        self.fluid.element[self.y][self.x] = Ele::Fluid;
        if let Some(idx) = self.fluid.boundaries.iter().position(
            |vec| *vec == Vector::construct(self.x, self.y)
        ) {
            self.fluid.boundaries.remove(idx);
        }
    }

    /// returns the Ele type of the the selected grid cell, able to see anywhere
    /// in the fluid grid.  this function is called with relative indexing, and
    /// will return the a solid Static boundary if attempted to peek into a cell
    /// that is out of bounds
    pub fn peek_element_here(&self, dx: isize, dy: isize) -> Ele {
        let (nx, ny): (usize, usize) = self.index(dx, dy);
        if self.fluid.inbounds(nx, ny) {
            self.fluid.element[ny][nx]
        } else {
            Ele::Static
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

    /// operates fundamentally in the same way as <peek_velocity()> but instead
    /// returns a mutable reference to the specific cell's velocity
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

    /// simply converts relative coordinates into valid, absolute indicies in
    /// the fluid's grid. this function gets called a lot of times, so it is 
    /// converted into x86 to ensure it's quick quick
    #[cfg_attr(not(target_arch = "x86_64"), allow(dead_code))]
    #[cfg(target_arch = "x86_64")]
    pub fn index(&self, dx: isize, dy: isize) -> (usize, usize) {
        let nx: usize;
        let ny: usize;

        unsafe {
            arch::asm!(
                "mov {tx}, {x}",
                "add {tx}, {dx}",
                "mov {nx}, {tx}",
                
                "mov {ty}, {y}",
                "add {ty}, {dy}",
                "mov {ny}, {ty}",

                tx = out(reg) _,
                ty = out(reg) _,
                nx = out(reg) nx,
                ny = out(reg) ny,
                x = in(reg) self.x as isize,
                y = in (reg) self.y as isize,
                dx = in(reg) dx,
                dy = in(reg) dy,
                options(nostack, nomem),
            );
        }

        (nx, ny)
    }
    
    #[cfg(not(target_arch = "x86_64"))]
    pub fn index(&self, dx: isize, dy: isize) -> (usize, usize) {
        let nx: usize = ((self.x as isize) + dx) as usize;
        let ny: usize = ((self.y as isize) + dy) as usize;
        (nx, ny)
    }
}
