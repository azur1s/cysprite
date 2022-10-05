pub mod state;

use macroquad::prelude::*;

#[macroquad::main("Cysprite")]
async fn main() {
    let mut state = state::State::new();

    loop {
        state.update();

        egui_macroquad::draw();
        next_frame().await
    }
}
