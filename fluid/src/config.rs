


use macroquad::prelude::*;



// static WIDTH: usize = 60;
// static HEIGHT: usize = 20;
// static CELL_SIZE: f32 = 35.0;
// static WIDTH: usize = 60 * 7;
// static HEIGHT: usize = 20 * 7;
// static CELL_SIZE: f32 = 35.0 / 7.0;
static WIDTH: usize = 60 * 5;
static HEIGHT: usize = 20 * 5;
static CELL_SIZE: f32 = 35.0 / 5.0;
// static WIDTH: usize = 80 * 10;
// static HEIGHT: usize = 20 * 10;
// static CELL_SIZE: f32 = 35.0 / 10.0;
// static WIDTH: usize = 60 * 22;
// static HEIGHT: usize = 20 * 22;
// static CELL_SIZE: f32 = 35.0 / 22.0;
// static WIDTH: usize = 80 * 35;
// static HEIGHT: usize = 20 * 35;
// static CELL_SIZE: f32 = 40.0 / 35.0;
static OVERRELAXATION: f32 = 1.93;
static ITERS: usize = 175;
static DELTA_T: f32 = 0.25;
static SOURCE_V: f32 = 50.0;
static VISUAL_MOD: f32 = 4.0;
static GRID_SIZE: f32 = 5.0;
static VORT_CONF_EPSILON: f32 = 0.15;
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
    pub source_velocity: f32,
    pub visual_modifier: f32,
    pub grid_size: f32,
    pub epsilon: f32,
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
            source_velocity: SOURCE_V,
            visual_modifier: VISUAL_MOD,
            grid_size: GRID_SIZE,
            epsilon: VORT_CONF_EPSILON,
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
        Self::Pause 
    }

    pub fn rotate(&self) -> State {
        match self {
            Self::Simulation => Self::Pause,
            Self::Pause      => Self::Simulation,
        }
    }
}

#[allow(dead_code)]
#[derive(PartialEq)]
pub enum VisualMode {
    Gradient,
    Vector,
    Other,
    Blank,
}

#[allow(dead_code)]
impl VisualMode {
    pub fn new() -> VisualMode {
        Self::Gradient
    }

    pub fn rotate(&self) -> VisualMode {
        match self {
            Self::Gradient => Self::Vector,
            Self::Vector   => Self::Other,
            Self::Other    => Self::Blank,
            Self::Blank    => Self::Gradient,
        }
    }
}
