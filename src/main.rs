mod board;
mod player;
mod ui;
mod game;


fn main() {
    // game::_setup_game();
    let g = game::Game::new_game();
    let ui = ui::example_gui(g);
    println!("ui: {:?}", ui);
}
