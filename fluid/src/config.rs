


use macroquad::prelude::*;



// static WIDTH: usize = 60;
// static HEIGHT: usize = 20;
// static CELL_SIZE: f32 = 35.0;
// static WIDTH: usize = 60 * 3;
// static HEIGHT: usize = 20 * 3;
// static CELL_SIZE: f32 = 35.0 / 3.0;
// static WIDTH: usize = 80 * 10;
// static HEIGHT: usize = 20 * 10;
// static CELL_SIZE: f32 = 35.0 / 10.0;
static WIDTH: usize = 80 * 22;
static HEIGHT: usize = 20 * 22;
static CELL_SIZE: f32 = 35.0 / 22.0;
static OVERRELAXATION: f32 = 1.93;
static ITERS: usize = 150;
static DELTA_T: f32 = 0.25;
// static OVERRELAXATION: f32 = 1.0;
// static ITERS: usize = 160;
// static DELTA_T: f32 = 0.01;

pub struct Config {
    pub x: usize,
    pub y: usize,
    pub overrelaxation: f32,
    pub cell_size: f32,
    pub iters: usize,
    pub delta_t: f32,
}

impl Config {
    pub fn new() -> Config {
        Config {
            x: WIDTH,
            y: HEIGHT,
            overrelaxation: OVERRELAXATION,
            cell_size: CELL_SIZE,
            iters: ITERS,
            delta_t: DELTA_T,
        }
    }
}

pub fn configuration() -> Conf {
    Conf {
        window_title: String::from("Fluids flowy flowy"),
        window_height: (HEIGHT as f32 * CELL_SIZE) as i32,
        window_width: (WIDTH as f32 * CELL_SIZE) as i32,
        window_resizable: false,
        ..Default::default()
    }
}

#[derive(PartialEq)]
pub enum State {
    Simulation,
    Pause,
}

impl State {
    pub fn new() -> State {
        State::Simulation
    }

    pub fn rotate(&self) -> State {
        match self {
            Self::Simulation => Self::Pause,
            Self::Pause      => Self::Simulation,
        }
    }
}
