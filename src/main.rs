pub mod debug_ui;
pub mod game;
pub mod misc;
pub mod renderer;
pub mod utils;

use crate::game::Game;

fn main() {
    env_logger::init();

    let game = Game::new();
    game.run_loop();
}
