slint::include_modules!();

use std::{rc::Rc, sync::{Arc, Mutex}};

use slint::{ModelRc};

use crate::game;
use slint::VecModel;


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
                game.game_run(&ui);
            }
        }
    });

    ui.run()
}

pub fn update_ui_board(board: Vec<Vec<i8>>, ui: &AppWindow) {
    let mut new_board = vec![];
    for i in 0..6 {
        new_board.push(vec![]);
        for j in 0..7 {
            if board[i][j] == -1 {
                new_board[i].push(2);
            } else {
                new_board[i].push(board[i][j] as i32);
            }
        }
    }
    let vm_0 = ModelRc::new(Rc::new(VecModel::from(new_board[0].clone())));
    let vm_1 = ModelRc::new(Rc::new(VecModel::from(new_board[1].clone())));
    let vm_2 = ModelRc::new(Rc::new(VecModel::from(new_board[2].clone())));
    let vm_3 = ModelRc::new(Rc::new(VecModel::from(new_board[3].clone())));
    let vm_4 = ModelRc::new(Rc::new(VecModel::from(new_board[4].clone())));
    let vm_5 = ModelRc::new(Rc::new(VecModel::from(new_board[5].clone())));

    let mr = vec![vm_0, vm_1, vm_2, vm_3, vm_4, vm_5];
    let mr = ModelRc::new(Rc::new(VecModel::from(mr)));

    ui.set_board(mr);
}