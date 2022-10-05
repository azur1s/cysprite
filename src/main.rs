pub mod state;

use macroquad::prelude::*;
use discord_rpc_client::Client;

#[macroquad::main("Cysprite")]
async fn main() {
    let mut state = state::State::new();

    let mut drpc = Client::new(1027289047779000330);
    println!("Starting discord RPC client");
    drpc.start();
    let r = drpc.set_activity(|act| {
        act
            .assets(|a| {
                a
                    .large_text("Cysprite")
            })
            .timestamps(|t| { t.start(chrono::Utc::now().timestamp() as u64) })
            .details("Painting pixels")
            .state("Working on some art")
    });
    if let Err(e) = r {
        println!("Discord RPC error: {}", e);
    } else {
        println!("Discord RPC startup success");
    }

    loop {
        state.update();

        egui_macroquad::draw();
        next_frame().await
    }
}
