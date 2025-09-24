use crate::{game::Game};

mod entity;
mod game;
mod world;

mod systems;

mod net;

fn main() {
    // init game
    let mut game = Game::new(256, 256);

    // start server

    // start tick
    game.tick();
}
