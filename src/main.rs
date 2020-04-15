#[macro_use]
extern crate log;
extern crate env_logger;

mod set;
mod game;
mod input;


fn main() {
    env_logger::init();
    let mut game = game::Game::initialize();
    game.begin_playing();
}
