pub mod ui;
pub mod game;
pub mod misc;
pub mod renderer;
pub mod utils;
pub mod physic;

use crate::game::Game;

fn main() {
    env_logger::init();

    let game = Game::new();
    game.run_loop();
}
