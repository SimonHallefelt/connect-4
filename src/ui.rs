slint::include_modules!();

use std::{rc::Rc, sync::{Arc, Mutex}};
use slint::{Model, ModelRc, VecModel, Timer, TimerMode};

use crate::game;


pub fn example_gui(game: game::Game) -> Result<(), slint::PlatformError> {
    let g = Arc::new(Mutex::new(game));
    let ui = Arc::new(AppWindow::new()?);
    
    make42pieces(ui.clone());

    let ui_time = Arc::clone(&ui);
    let g_time = Arc::clone(&g);
    let timer = Timer::default();
    timer.start(TimerMode::Repeated, std::time::Duration::from_millis(10), move || {
        // println!("This will be printed every 10ms.");
        let game = g_time.lock().unwrap();
        update_ui_board(game.get_board(), Arc::clone(&ui_time));
    });

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
        let g = Arc::clone(&g);
        move || {
            game::game_run(Arc::clone(&g));
        }
    });

    ui.on_clicked_board_box(move |index|{
        println!("clicked board box: {}", index);
        if let Ok(mut game) = g.lock() {
            if game.get_running(){
                println!("game is running, move selected: {}", index);
            }
        }
    });

    ui.run()
}



fn make42pieces(ui: Arc<AppWindow>) {
    let old_pieces = ui.get_pieces().iter().map(|p| p.clone()).collect::<Vec<pieceData>>();
    let piece: pieceData = old_pieces[0].clone();
    let pieces = ModelRc::new(Rc::new(VecModel::from(vec![piece; 42])));
    ui.set_pieces(pieces.into());
}



pub fn update_ui_board(board: Vec<Vec<i8>>, ui: Arc<AppWindow>) {
    // old board update
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

    ui.set_board(mr.into());

    // new board update
    // let mut pieces = ui.get_pieces().iter().map(|p| p.clone()).collect::<Vec<pieceData>>();
    // for i in 0..6 {
    //     for j in 0..7 {
    //         let index = i * 7 + j;
    //         if board[i][j] == 0 {
    //             pieces[index].player_piece = 0;
    //         } else if board[i][j] == 1 {
    //             pieces[index].player_piece = 1;
    //         } else if board[i][j] == -1 {
    //             pieces[index].player_piece = 2;
    //         }
    //     }
    // }
    // let pieces = ModelRc::new(Rc::new(VecModel::from(pieces)));
    // ui.set_pieces(pieces.into());
}