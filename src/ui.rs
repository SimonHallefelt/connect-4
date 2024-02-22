slint::include_modules!();

use std::{rc::Rc, sync::{Arc, Mutex}, thread};

use slint::ModelRc;

use crate::game;
use slint::VecModel;


pub fn example_gui(game: game::Game) -> Result<(), slint::PlatformError> {
    let g = Arc::new(Mutex::new(game));
    let ui = Arc::new(AppWindow::new()?);
    
    ui.on_set_player_type({
        let ui = Arc::clone(&ui);
        let g = Arc::clone(&g);
        move || {
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
        let ui = Arc::clone(&ui);
        let g = Arc::clone(&g);
        move || {
            game::game_run(Arc::clone(&g));
            board_update_after_time(Arc::clone(&g), Arc::clone(&ui));
        }
    });

    ui.run()
}



fn board_update_after_time(g: Arc<Mutex<game::Game>>, ui: Arc<AppWindow>) {
    loop {
        std::thread::sleep(std::time::Duration::from_millis(50));
        let game = g.lock().unwrap();
        update_ui_board(game.get_board(), Arc::clone(&ui));
        if !game.get_running() {
            break;
        }
    }
}



pub fn update_ui_board(board: Vec<Vec<i8>>, ui: Arc<AppWindow>) {
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