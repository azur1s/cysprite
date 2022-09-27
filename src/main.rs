use macroquad::prelude::*;

mod grid;
mod util;
mod state;

use state::State;

#[macroquad::main("harcana - pixel art tool")]
async fn main() {

    let mut state = State::new();

    loop {
        state.update();
        egui_macroquad::draw();
        next_frame().await
    }
}
