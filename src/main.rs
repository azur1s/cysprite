use macroquad::prelude::*;

mod util;

mod grid;
mod undo;
mod status_message;

mod state;

use state::State;

#[macroquad::main("harcana")]
async fn main() {

    let mut state = State::new();
    state.init();

    loop {
        state.update();
        egui_macroquad::draw();
        next_frame().await
    }
}
