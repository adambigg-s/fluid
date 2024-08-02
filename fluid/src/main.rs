


mod fluid;
mod utils;
mod config;



use std::{env, time::Duration};
use macroquad::prelude::*;



use fluid::Fluid;
use config::{configuration, Config};



#[macroquad::main(configuration)]
async fn main() {
    println!("Hello, fluids!");

    env::set_var("RUST_BACKTRACE", "1");

    let config: Config = Config::new();
    let mut fluid: Fluid = Fluid::new(&config);

    fluid.assert_bounds();
    fluid.assert_emitter();
    fluid.print_cli();
    // fluid.update_fluid();
    // fluid.update_fluid();
    // fluid.update_fluid();
    // fluid.print_cli();

    loop {
        clear_background(Color::from_hex(0x5a5255));

        fluid.draw(true);

        if is_mouse_button_down(MouseButton::Left) {
            fluid.update_fluid();
        } else if is_mouse_button_pressed(MouseButton::Right) {
            fluid.update_fluid();
        }

        next_frame().await;
        std::thread::sleep(Duration::from_millis(1));
    }
}

