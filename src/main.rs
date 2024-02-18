mod board;
mod player;
mod gui;
mod game;


fn main() {
    // game::_setup_game();
    let g = game::Game::new_game();
    let gui = gui::example_gui(g);
    println!("gui: {:?}", gui);
}
