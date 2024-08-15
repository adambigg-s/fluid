


mod config;
mod fluid;
mod utils;
mod clone;
mod source;
mod fluidapi;
mod units;
mod legacy;



use macroquad::prelude::*;
use std::{env, time::Duration};



use config::{configuration, Config, State, VisualMode};
use fluid::Fluid;
use utils::{Vector, place_tool};



#[macroquad::main(configuration)]
async fn main() {
    // used for debugging and backtracing panics
    env::set_var("RUST_BACKTRACE", "full");
    env::set_var("CARGO_PROFILE_RELEASE_DEBUG", "true");

    let config: Config = Config::new();
    let mut fluid: Fluid = Fluid::construct(&config);
    let mut state: State = State::new();
    let mut display: VisualMode = VisualMode::new();
    let mut p_mouse: Option<Vector<f32>> = None;
    let mut diag: bool = true;

    println!("Grid Size: {}", fluid.x * fluid.y);

    fluid.assert_boundary_conditions();
    // runs some additional iterations thru the projection phase of the grid-solver. 
    // this is used to just sort of "get some slack" out of the matrix, as the startup 
    // phase takes the longest to converge in most cases
    for _ in 0..3 {
        fluid.update_fluid(true, false, false, false);
    }

    // starts loop for update-draw cycle
    loop {
        clear_background(Color::from_hex(0x121212));

        // visual enum used to select different visuals. blank mode is used to remove draw-loop overhead
        // and to allow faster iterations for long running sims
        match display {
            VisualMode::Gradient   => {
                fluid.display(true, false, false, 0.4, 0.7, 1, false, true);
            }
            VisualMode::Vector     => {
                fluid.display(true, true, false, 0.4, 0.7, 1, true, false);
            }
            VisualMode::Other      => {
                fluid.display(true, false, false, 0.4, 0.7, 1, false, true);
                fluid.streamline(30, 10, 100, 0.07, 0.2);
            }
            VisualMode::Streamline => {
                fluid.display(false, false, false, 1.0, 10.0, 1000, false, true);
                fluid.streamline(30, 8, 155, 0.05, 0.2);
            }
            VisualMode::Blank      => {}
        } if is_key_pressed(KeyCode::V) {
            display = display.rotate();
        }

        if is_key_pressed(KeyCode::Key6) {
            fluid.visual_modifier = 0.0_f32.max(fluid.visual_modifier - 0.1);
        } else if is_key_pressed(KeyCode::Key7) {
            fluid.visual_modifier = 100.0_f32.min(fluid.visual_modifier + 0.1);
        }

        // passes time on sim
        if state == State::Simulation {
            fluid.update_fluid(true, true, true, true);
        } 
        if is_key_pressed(KeyCode::P) {
            state = state.rotate();
        }

        // manual update of specific stages - used for debugging mainly to see where crashes / instabilty occur
        if is_mouse_button_pressed(MouseButton::Left) {
            fluid.update_fluid(true, false, false, false);
        } else if is_mouse_button_pressed(MouseButton::Right) {
            fluid.update_fluid(false, true, false, false);
        } else if is_key_down(KeyCode::B) {
            fluid.update_fluid(false, false, true, false);
        }

        // used to sort of "draw" boundaries 
        if is_key_down(KeyCode::W) {
            place_tool(&mut p_mouse, &mut fluid, "place");
        } else if is_key_down(KeyCode::D) {
            place_tool(&mut p_mouse, &mut fluid, "delete");
        } else {
            p_mouse = None;
        }

        if is_key_down(KeyCode::W) && is_key_down(KeyCode::F) {
            let (x, y) = mouse_position();
            fluid.fill_dfs(
                (x / fluid.cell_size) as usize, 
                (y / fluid.cell_size) as usize
            );
        }

        // resets all placed boundaries and current fluid state 
        if is_key_pressed(KeyCode::R) {
            fluid.reset();
        }

        if is_key_pressed(KeyCode::K) {
            diag = !diag;
        }
        if diag {
            draw_text(
                &format!("FPS: {}", get_fps()),
                30.0,
                20.0,
                20.0,
                RED,
            );
            draw_text(
                &format!("b.c. len: {}", fluid.boundaries.len()),
                30.0,
                40.0,
                20.0,
                RED,
            );
        }

        // awaits next frame, optional delay but usually set to 0 as the sims run slow anyway
        next_frame().await;
        std::thread::sleep(Duration::from_millis(0));
    }
}
