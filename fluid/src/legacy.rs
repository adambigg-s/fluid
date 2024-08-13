


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

pub fn get_color_vec(vec: &Vector<f32>, max: f32) -> Color {
    let mag = vec.magnitude();
    let clamped = mag.clamp(0.0, max);
    let normalized_mag = clamped / max;

    let r = (normalized_mag * 255.0) as u8;
    let g = 100;
    let b = ((1.0 - normalized_mag) * 255.0) as u8;

    Color::from_rgba(r, g, b, 200)
}

        if is_key_down(KeyCode::W) {
            let (x, y) = mouse_position();
            let (x, y) = (
                (x / fluid.cell_size) as usize,
                (y / fluid.cell_size) as usize,
            );
            fluid.assert_boundary_place(x, y);
        } 
        
    // pub fn assert_static_velocity(&mut self) {
    //     for stat in self.statics.clone() {
    //         let mut oo: Oo = Oo::construct(stat.x, stat.y, self);
    //         oo.set_velocity_zeros();
    //     }
    // }

    // pub fn assert_source_velocity(&mut self) {
    //     for sour in self.sources.clone() {
    //         let mut oo: Oo = Oo::construct(sour.x, sour.y, self);
    //         oo.set_velocity_polarized(oo.fluid.source_velocity, 0.0);
    //     }
    // }

    fn semi_lagrangian_advection_depricated(&mut self) {
        for y in 0..self.y {
            for x in 0..self.x {
                
                if self.element[y][x] != DiffEle::Fluid {
                    continue;
                }

                let prev_x =
                    (x as f32 - self.u[y][x] * self.delta_t / self.grid_size).round() as usize;
                let prev_y =
                    (y as f32 - self.v[y][x] * self.delta_t / self.grid_size).round() as usize;

                if self.inbounds(prev_x, prev_y) {
                    self.nu[y][x] = self.u[prev_y][prev_x];
                    self.nv[y][x] = self.v[prev_y][prev_x];
                }
            }
        }
        self.u.clone_from(&self.nu);
        self.v.clone_from(&self.nv);
    }
    
    fn double_lin_int_depricated(&self, x: f32, y: f32, field: &str) -> f32 {
        let field = match field {
            "u" => &self.u,
            "v" => &self.v,
            _   => {
                eprintln!("Error in field token");
                std::process::exit(99);
            }
        };

        let x = x.round() as usize;
        let y = y.round() as usize;

        let x = x.clamp(0, self.x-1);
        let y = y.clamp(0, self.y-1);

        field[y][x]
    }

*/
