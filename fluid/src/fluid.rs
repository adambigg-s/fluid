


use crate::config;
use crate::utils;
use crate::clone;
use crate::source;
use crate::fluidapi;



use macroquad::prelude::*;
use std::arch;



use config::Config;
use utils::{get_color_vec, Vector, get_directions};
use clone::Clone;
use source::Source;
use fluidapi::Oo;



/// union enum used to store state of grid's contained elements
#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(dead_code)]
pub enum Ele {
    /// Fluid is used to carry no additional information and is subject to all state changes
    Fluid,
    
    /// Static boundary maintains zero flux and asserts the presence of strictly tangential velocity.
    /// presence of Static implies with a wall or solid object 
    Static,
    
    /// Source holds a struct storing maintained velocity. this can be used to effectively set in/out-flow
    /// velocity at a controlled rate 
    Source(Source),
    
    /// Clone holds a struct carring relative indexing information pointing towards a cell to clone state.
    /// this effectively allows the effect of extending bounds indefinitely. 
    Clone(Clone),
}

impl Ele {
    #[allow(dead_code)]
    pub fn to_strslice(self) -> &'static str {
        match self {
            Self::Fluid     => "Fluid",
            Self::Static    => "Solid",
            Self::Source(_) => "Emitter",
            Self::Clone(_)  => "Match",
        }
    }

    /// returns true for Fluid and Clone - both of these cells are subject to effective divergence and 
    /// must be taken into account for calculations 
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

    pub element: Vec<Vec<Ele>>,

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

            // +1 in the x-direction to account for staggered grid 
            u: vec![vec![0.0; config.x + 1]; config.y],
            // +1 in the y-direction to account for staggered grid
            v: vec![vec![0.0; config.x]; config.y + 1],
            nu: vec![vec![0.0; config.x + 1]; config.y],
            nv: vec![vec![0.0; config.x]; config.y + 1],
            vorticity: vec![vec![0.0; config.x]; config.y],

            element: vec![vec![Ele::Fluid; config.x]; config.y],

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

    /// resets all state saves of the fluid struct to in the current draw-loop
    pub fn reset(&mut self) {
        self.u = vec![vec![0.0; self.x + 1]; self.y];
        self.v = vec![vec![0.0; self.x]; self.y + 1];
        self.nu = vec![vec![0.0; self.x + 1]; self.y];
        self.nv = vec![vec![0.0; self.x]; self.y + 1];
        self.element = vec![vec![Ele::Fluid; self.x]; self.y];
        self.boundaries = Vec::new();
        self.assert_boundary_conditions();
    }

    /// interacts specifically with indexing functions to ensure within bounds of fluid
    #[cfg_attr(not(target_arch = "x86_64"), allow(dead_code))]
    #[cfg(target_arch = "x86_64")]
    pub fn inbounds(&self, x: usize, y: usize) -> bool {
        let result: u8;
        
        unsafe {
            arch::asm!(
                "cmp {x}, {bx}",
                "jae 1f",
                "cmp {y}, {by}",
                "jae 1f",
                "mov {result}, 1",
                "jmp 2f",
                "1:",
                "mov {result}, 0",
                "2:",
                x      = in(reg)       x,
                y      = in(reg)       y,
                bx     = in(reg)       self.x,
                by     = in(reg)       self.y,
                result = out(reg_byte) result,
                options(nostack, nomem, preserves_flags),
            );
        }

        result != 0
    }

    #[cfg(not(target_arch = "x86_64"))]
    pub fn inbounds(&self, x: usize, y: usize) -> bool {
        x < self.x && y < self.y
    }
    
    pub fn assert_boundary_place(&mut self, x: usize, y: usize) {
        if self.inbounds(x, y) {
            let mut oo: Oo = Oo::construct(x, y, self);
            oo.set_here(Ele::Static);
        }
    }

    pub fn assert_boundary_delete(&mut self, x: usize, y: usize) {
        if self.inbounds(x, y) {
            let mut oo: Oo = Oo::construct(x, y, self);
            oo.remove_here();
        }
    }

    /// set intial boundary conditions at start of simulation - shoud only be called 
    /// one time per sim
    pub fn assert_boundary_conditions(&mut self) {
        let _xx = self.x;
        let _yy = self.y;

        // assert maintainable boundary conditions on 4 sides
        self.fill_right_border(Ele::Clone(Clone::construct(-1, 0)));
        self.fill_top_border(Ele::Static);
        self.fill_bot_border(Ele::Static);
        for i in 0..self.y {
            let mut oo = Oo::construct(0, i, self);
            if _yy * 5 / 11 < i && i < _yy * 6 / 11 {
                oo.set_here(Ele::Source(Source::construct(oo.fluid.source_velocity, 0.0)));
            }
            else {
                oo.set_here(Ele::Static);
            }
        }

        // add standard geometry
        self.create_circle(_xx / 5, _yy / 2, (_yy / 11) as f32);

        // apply boundary conditions to all elements initalized 
        self.enforce_boundary_conditions();
    }

    /// places circular geometry at a location in the simulation
    #[allow(dead_code)]
    fn create_circle(&mut self, center_x: usize, center_y: usize, radius: f32) {
        for y in 0..self.y {
            for x in 0..self.x {
                if (x as f32 - center_x as f32).powf(2.0) 
                    + (y as f32 - center_y as f32).powf(2.0) 
                    < radius.powf(2.0) 
                {
                    let mut oo = Oo::construct(x, y, self);
                    oo.set_here(Ele::Static);
                }
            }
        }
    }

    /// places rectangular geometry in the simulation
    #[allow(dead_code)]
    fn create_rectangle(&mut self, x0: usize, y0: usize, x1: usize, y1: usize) {
        for y in y0..y1 {
            for x in x0..x1 {
                let mut oo = Oo::construct(x, y, self);
                oo.set_here(Ele::Static);
            }
        }
    }

    #[allow(dead_code)]
    fn fill_top_border(&mut self, fill: Ele) {
        for x in 0..self.x {
            let mut oo = Oo::construct(x, 0, self);
            oo.set_here(fill);
        }
    }

    #[allow(dead_code)]
    fn fill_bot_border(&mut self, fill: Ele) {
        for x in 0..self.x {
            let mut oo = Oo::construct(x, self.y-1, self);
            oo.set_here(fill);
        }
    }

    #[allow(dead_code)]
    fn fill_right_border(&mut self, fill: Ele) {
        for y in 0..self.y {
            let mut oo = Oo::construct(self.x-1, y, self);
            oo.set_here(fill);
        }
    }

    #[allow(dead_code)]
    fn fill_left_border(&mut self, fill: Ele) {
        for y in 0..self.y {
            let mut oo = Oo::construct(0, y, self);
            oo.set_here(fill);
        }
    }

    /// fills in a closed portion of the fluid grid with additional, non-simulated boundaries.
    /// while mostly marginal, should be used as often as possible to improve performance.
    /// as explained in the function, any cells filled in via dfs are literally not ever simulated 
    /// and thus the more the merrier in terms of speed 
    pub fn fill_dfs(&mut self, x: usize, y: usize) {
        let mut stack = vec![(x, y)];
        let mut seen = vec![vec![false; self.x]; self.y];

        while let Some((x, y)) = stack.pop() {
            if self.element[y][x] == Ele::Static || seen[y][x] {
                continue;
            }

            seen[y][x] = true;
            // explicitly changes element matrix without using Oo wrapper api. this is intentional and desired 
            // desired behavior becuase we can operate with the assumption that a fill algorithm will never 
            // make meaninful impact on outer bcs via stokes
            self.element[y][x] = Ele::Static;

            for (dx, dy) in get_directions() {
                let nx = (x as isize + dx) as usize;
                let ny = (y as isize + dy) as usize;
                if nx< self.x && ny < self.y && !seen[ny][nx] {
                    stack.push((nx, ny));
                }
            }
        }
    }
    
    /// prints fluid to a text file - slow and should only be used for debugging on small grids
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
                    Ele::Fluid     => continue,
                    Ele::Static    => Color::from_hex(0x000000),
                    Ele::Source(_) => Color::from_hex(0x1b85b8),
                    Ele::Clone(_)  => Color::from_hex(0x559e83),
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
                if !draw_bounds && self.element[y][x] != Ele::Fluid {
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

    pub fn streamline(&self, spacing_x: usize, spacing_y: usize, max_steps: usize, step_size: f32, thickness: f32) {
        let mut streamlines: Vec<Vec<Vector<f32>>> = Vec::new();

        for y in (0..self.y).step_by(spacing_y) {
            for x in (0..self.x).step_by(spacing_x) {
                let seed = Vector::construct(x as f32, y as f32);
                let streamline = self.compute_streamline(seed, max_steps, step_size);
                streamlines.push(streamline);
            }
        }

        for streamline in streamlines {
            let len = streamline.len();
            if len < 1 { continue; }
            for idx in 0..(len-1) {
                draw_line(
                    streamline[idx].x * self.cell_size, 
                    streamline[idx].y * self.cell_size, 
                    streamline[idx+1].x * self.cell_size, 
                    streamline[idx+1].y * self.cell_size, 
                    thickness, 
                    WHITE,
                );
            }
        }
    }

    fn compute_streamline(&self, seed: Vector<f32>, max_steps: usize, step_size: f32) -> Vec<Vector<f32>> {
        let mut streamline = Vec::new();
        let (mut x, mut y) = (seed.x, seed.y);

        for _ in 0..max_steps {
            streamline.push(
                Vector::construct(x, y)
            );
            let u = self.double_lin_int(x, y, "u");
            let v = self.double_lin_int(x, y, "v");

            x += u * step_size;
            y += v * step_size;

            if !self.inbounds(x as usize, y as usize) {
                break;
            }
        }

        streamline
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
            // projection is done last in order to maintain clearest view as display will be called after this
            self.projection_gauss_seidel();
        }
    }

    fn projection_gauss_seidel(&mut self) {
        for _ in 0..self.iters {
            
            for y in 0..self.y {
                for x in 0..self.x {
                    
                    if self.element[y][x] != Ele::Fluid {
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

    #[cfg(target_arch = "never")]
    fn apply_vorticity_confinement(&mut self) {
        self.compute_vorticity();

        for i in 1..self.y-1 {
            for j in 1..self.x-1 {
                let half: f32 = 0.5;
                let gwx: f32;
                let gwy: f32;

                unsafe {
                    arch::asm!(
                        "movss xmm0, [{vorticity1}]",
                        "movss xmm1, [{vorticity2}]",
                        "subss xmm0, xmm1",
                        "mulss xmm0, [{half}]",
                        "movss [{grad_w_x}], xmm0",

                        "movss xmm1, [{vorticity3}]",
                        "movss xmm2, [{vorticity4}]",
                        "subss xmm1, xmm2",
                        "mulss xmm1, [{half}]",
                        "movss [{grad_w_y}], xmm1",

                        grad_w_x   = out(xmm_reg) gwx,
                        grad_w_y   = out(xmm_reg) gwy,
                        vorticity1 = in(reg)      &self.vorticity[i][j+1],
                        vorticity2 = in(reg)      &self.vorticity[i][j-1],
                        vorticity3 = in(reg)      &self.vorticity[i+1][j],
                        vorticity4 = in(reg)      &self.vorticity[i-1][j],
                        half       = in(reg)      &half,
                        options(nostack)
                    );
                }

                let magnitude: f32 = Vector::construct(gwx, gwy).magnitude();
                if magnitude > 1e-6 {
                    self.apply_vorticity_force(gwx, gwy, magnitude, i, j);
                }
            }
        }
    }

    #[cfg(target_arch = "never")]
    fn apply_vorticity_force(&mut self, grad_w_x: f32, grad_w_y: f32, magnitude: f32, i: usize, j: usize) {
        let force_x: f32;
        let force_y: f32;
        
        unsafe {
            arch::asm!(
                "movss xmm0, [{gwx}]",
                "movss xmm1, [{gwy}]",
                "movss xmm2, [{mag}]",
                "movss xmm3, [{vort}]",
                
                "divss xmm0, xmm2",
                "divss xmm1, xmm2",
                
                "mulss xmm0, xmm3",
                "mulss xmm1, xmm3",

                "mulss xmm0, [{epsilon}]",
                "mulss xmm1, [{epsilon}]",

                "movss [{fx}], xmm0",
                "movss [{fy}], xmm1",

                fx      = out(xmm_reg) force_x,
                fy      = out(xmm_reg) force_y,
                gwx     = in(reg)      &grad_w_x,
                gwy     = in(reg)      &grad_w_y,
                mag     = in(reg)      &magnitude,
                epsilon = in(reg)      &self.epsilon,
                vort    = in(reg)      &self.vorticity[i][j],
                options(nostack)
            );
            
            self.u[i][j] += force_x * self.delta_t;
            self.v[i][j] += force_y * self.delta_t;
        }
    }

    #[cfg(target_arch = "x86_64")]
    fn apply_vorticity_confinement(&mut self) {
        self.compute_vorticity();

        for i in 1..self.y-1 {
            for j in 1..self.x-1 {

                let grad_w_x: f32 = (self.vorticity[i][j+1] - self.vorticity[i][j-1]) * 0.5;
                let grad_w_y: f32 = (self.vorticity[i+1][j] - self.vorticity[i-1][j]) * 0.5;

                let magnitude = Vector::construct(grad_w_x, grad_w_y).magnitude();
                if magnitude > 1e-6 {
                    let nx: f32 = grad_w_x / magnitude;
                    let ny: f32 = grad_w_y / magnitude;

                    let force_x: f32 = self.epsilon * (ny * self.vorticity[i][j]);
                    let force_y: f32 = -self.epsilon * (nx * self.vorticity[i][j]);
                    
                    self.u[i][j] += force_x * self.delta_t;
                    self.v[i][j] += force_y * self.delta_t;
                }
            }
        }
    }

    #[cfg_attr(not(target_arch = "x86_64"), allow(dead_code))]
    #[cfg(target_arch = "x86_64")]
    fn compute_vorticity(&mut self) {
        for i in 1..self.y-1 {
            for j in 1..self.x-1 {
                let mut _dwdy: f32;
                let mut _dudx: f32;
                let half: f32 = 0.5;

                unsafe {
                    arch::asm!(
                        "movss xmm0, [{u_plus1_j}]",
                        "movss xmm1, [{u_minus1_j}]",
                    
                        "subss xmm0, xmm1",

                        "mulss xmm0, [{half}]",

                        "movss {dwdy}, xmm0",

                        "movss xmm2, [{v_i_jplus1}]",
                        "movss xmm3, [{v_i_jminus1}]",

                        "subss xmm2, xmm3",
                        "mulss xmm2, [{half}]",
                        "movss {dudx}, xmm2",
                        "subss xmm0, xmm2",

                        "movss [{vorticity_ij}], xmm0",

                        dwdy         = out(xmm_reg) _dwdy,
                        dudx         = out(xmm_reg) _dudx,
                        u_plus1_j    = in(reg)      &self.u[i + 1][j],
                        u_minus1_j   = in(reg)      &self.u[i - 1][j],
                        v_i_jplus1   = in(reg)      &self.v[i][j + 1],
                        v_i_jminus1  = in(reg)      &self.v[i][j - 1],
                        half         = in(reg)      &half,
                        vorticity_ij = in(reg)      &mut self.vorticity[i][j],
                        options(nostack),
                    );
                }
            }
        }
    }    
    
    #[cfg(not(target_arch = "x86_64"))]
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
                Ele::Static       => {
                    oo.set_velocity_zeros();
                }
                Ele::Source(sour) => {
                    oo.set_velocity_polarized(sour.velocity.x, sour.velocity.y);
                }
                Ele::Clone(clo)   => {
                    oo.set_velocity_matched(clo.master.x, clo.master.y);
                }
                _                 => {}
            }
        }
    }
}

