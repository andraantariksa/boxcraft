pub mod game;
pub mod misc;
pub mod physic;
pub mod plugin;
pub mod renderer;
pub mod ui;
pub mod utils;

use crate::game::Game;

fn main() {
    env_logger::init();

    let game = Game::new();
    game.run_loop();
}
