


mod config;
mod fluid;
mod utils;
mod clone;
mod fluidapi;



use macroquad::prelude::*;
use std::{env, time::Duration};



use config::{configuration, Config, State, VisualMode};
use fluid::Fluid;



#[macroquad::main(configuration)]
async fn main() {
    println!("Hello, fluids!");

    env::set_var("RUST_BACKTRACE", "1");

    let config: Config = Config::new();
    let mut fluid: Fluid = Fluid::construct(&config);
    let mut state: State = State::new();
    let mut display: VisualMode = VisualMode::new();

    println!("Grid Size: {}", fluid.x * fluid.y);

    fluid.assert_boundary_conditions();
    for _ in 0..15 {
        fluid.update_fluid(true, false, false);
    }

    loop {
        clear_background(Color::from_hex(0x000000));

        match display {
            VisualMode::Gradient => {
                fluid.display(true, false, false, 0.4, 0.7, 1, false, true);
            }
            VisualMode::Vector   => {
                fluid.display(true, false, false, 0.4, 0.7, 5, true, false);
            }
            VisualMode::Other    => {
                fluid.display(true, false, true, 0.4, 0.8, 1, true, false);
            }
            VisualMode::Blank    => {}
        } if is_key_pressed(KeyCode::V) {
            display = display.rotate();
        }

        if state == State::Simulation {
            fluid.update_fluid(true, true, true);
        } 
        if is_key_pressed(KeyCode::P) {
            state = state.rotate();
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            fluid.update_fluid(true, false, false);
        } else if is_mouse_button_pressed(MouseButton::Right) {
            fluid.update_fluid(true, true, true);
        }

        if is_key_down(KeyCode::W) {
            let (x, y) = mouse_position();
            let (x, y) = (
                (x / fluid.cell_size) as usize,
                (y / fluid.cell_size) as usize,
            );
            fluid.assert_boundary_place(x, y);
        } else if is_key_pressed(KeyCode::R) {
            fluid.reset();
        }

        draw_text(
            &format!("FPS: {}", get_fps()),
            30.0,
            20.0,
            20.0,
            RED,
        );

        next_frame().await;
        std::thread::sleep(Duration::from_millis(0));
    }
}
