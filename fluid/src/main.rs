


mod config;
mod fluid;
mod utils;



use macroquad::prelude::*;
use std::{env, time::Duration};



use config::{configuration, Config, State};
use fluid::Fluid;



#[macroquad::main(configuration)]
async fn main() {
    println!("Hello, fluids!");

    env::set_var("RUST_BACKTRACE", "1");

    let config: Config = Config::new();
    let mut fluid: Fluid = Fluid::construct(&config);
    let mut state: State = State::new();
    let mut accesses: i128 = 0;

    state = state.rotate();

    println!("Grid Size: {}", fluid.x * fluid.y);

    fluid.assert_boundary_conditions();
    for _ in 0..15 {
        fluid.update_fluid(true, false, false);
    }
    fluid.print_cli();

    loop {
        clear_background(BLACK);

        fluid.display(true, false, false, 0.6, 1.0, 1, false, true);

        if state == State::Simulation {
            fluid.update_fluid(true, true, true);
            accesses += (fluid.iters * fluid.x * fluid.y) as i128;
        }
        if is_key_pressed(KeyCode::P) {
            state = state.rotate();
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            fluid.update_fluid(true, false, false);
            accesses += config.iters as i128;
        } else if is_mouse_button_pressed(MouseButton::Right) {
            fluid.update_fluid(true, true, true);
            accesses += config.iters as i128;
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
            &format!("Matrix Accesses: {}", accesses),
            30.0,
            40.0,
            20.0,
            RED,
        );
        // draw_text(
        //     &format!("FPS: {}", get_fps()),
        //     30.0,
        //     20.0,
        //     20.0,
        //     RED,
        // );

        next_frame().await;
        std::thread::sleep(Duration::from_millis(0));
    }
}
