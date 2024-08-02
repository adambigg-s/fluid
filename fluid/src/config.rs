


use macroquad::prelude::*;



// static WIDTH:          usize = 20;
// static HEIGHT:         usize = 10;
static WIDTH:          usize = 400;
static HEIGHT:         usize = 60;
static OVERRELAXATION: f32   = 1.0;
static CELL_SIZE:      f32   = 5.0;
static ITERS:          usize = 10;
static DELTA_T:        f32   = 0.04;

pub struct Config {
    pub x:              usize,
    pub y:              usize,
    pub overrelaxation: f32,
    pub cell_size:      f32,
    pub iters:          usize,
    pub delta_t:        f32,
}

impl Config {
    pub fn new() -> Config {
        Config {
            x:              WIDTH,
            y:              HEIGHT,
            overrelaxation: OVERRELAXATION,
            cell_size:      CELL_SIZE,
            iters:          ITERS,
            delta_t:        DELTA_T,
        }
    }
}

pub fn configuration() -> Conf {
    Conf {
        window_title:     String::from("Fluids flowy flowy"),
        window_height:    (HEIGHT as f32 * CELL_SIZE) as i32,
        window_width:     (WIDTH  as f32 * CELL_SIZE) as i32,
        window_resizable: false,
        ..Default::default()
    }
}
