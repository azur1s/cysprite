use macroquad::prelude::*;

mod util;

mod grid;
mod undo;
mod status_message;

mod state;

use state::State;

#[macroquad::main("cysprite")]
async fn main() {

    let mut state = State::new();
    state.init();

    loop {
        state.update();
        egui_macroquad::draw();

        std::thread::sleep(std::time::Duration::from_millis(
            if state.is_idle { 60 } else { 15 },
        ));
        next_frame().await
    }
}
