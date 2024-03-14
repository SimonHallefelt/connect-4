mod board;
mod player;
mod ui;
mod game;


fn main() {
    // game::_setup_game(); // run the game without the UI
    let g = game::Game::new_game();
    let ui = ui::start_ui(g);
    println!("ui: {:?}", ui);
}
