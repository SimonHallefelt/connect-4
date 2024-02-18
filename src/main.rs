use rand::Rng;
use std::time::Instant;

mod board;
mod player;
mod gui;
mod game;

fn new_game() {
    // Add implementation for new_game function here
}

fn main() {
    // game::setup_game();
    let g = game::Game::new_game();
    let gui = gui::example_gui(g);
    println!("gui: {:?}", gui);
}




