slint::include_modules!();

use std::sync::{Arc, Mutex};

//import player
use crate::game;

// make a gui whit a button that increases a counter
pub fn example_gui(game: game::Game) -> Result<(), slint::PlatformError> {
    let g = Arc::new(Mutex::new(game));
    let ui = AppWindow::new()?;

    ui.on_set_player_type({
        let ui_handle = ui.as_weak();
        let g = Arc::clone(&g);
        move || {
            let ui = ui_handle.unwrap();
            println!("ui: player: {}, player_type: {}", ui.get_player(), ui.get_player_type());
            let player = ui.get_player() as i8;
            let player_type = ui.get_player_type() as i8;
            if player == 1 {
                ui.set_player_1_type(player_type as i32);
                if let Ok(mut game) = g.lock() {
                    game.update_player(1, player_type);
                }
            } else {
                ui.set_player_2_type(player_type as i32);
                if let Ok(mut game) = g.lock() {
                    game.update_player(-1, player_type);
                }
            }
        }
    });

    ui.on_request_run_game({
        let ui_handle = ui.as_weak();
        let g = Arc::clone(&g);
        move || {
            let ui = ui_handle.unwrap();
            if let Ok(game) = g.lock() {
                game.game_run();
            }
        }
    });

    ui.run()
}