pub mod game;
pub mod input;
pub mod physics;
pub mod renderer;

use crate::game::Game;
use crate::input::InputManager;
use renderer::Renderer;
use std::rc::Rc;
use std::time::Instant;

fn main() {
    env_logger::init();

    let game = Game::new();
    game.run_loop();
}
