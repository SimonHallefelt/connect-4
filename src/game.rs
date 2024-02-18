

use std::time::Instant;
use rand::Rng;
use slint::ComponentHandle;

use crate::player;
use crate::board;
use crate::ui;
use crate::ui::AppWindow;


pub struct Game {
    p1: player::Player,
    p2: player::Player,
    // board: board::Board,
    players_turn: i8,
    d1: u128,
    d2: u128,
    d1_max: u128,
    d2_max: u128,
    ub: i8,
}

impl Game {
    pub fn new_game() -> Game {
        Game {
            p1: player::_select_player_in_code( 1, 0),
            p2: player::_select_player_in_code(-1, 0),
            // board: board::new_board(),
            players_turn: starting_player(),
            d1: 0,
            d2: 0,
            d1_max: 0,
            d2_max: 0,
            ub: 0,
        }
    }

    pub fn update_player(&mut self, player: i8, player_type: i8) {
        if player == 1 {
            self.p1 = player::_select_player_in_code(player, player_type);
        } else {
            self.p2 = player::_select_player_in_code(player, player_type);
        }
    }

    pub fn game_run(&self, _ui: slint::Weak<AppWindow>) {
        let ui_2: AppWindow = _ui.unwrap();
        run_ui(self.p1.clone(), self.p2.clone(), ui_2.as_weak());
    }
}




fn starting_player() -> i8 {
    let mut rng = rand::thread_rng();
    match rng.gen_range(0..2) {
        1 => return 1,
        _ => return -1,
    }
}

fn run_ui(p1: player::Player, p2: player::Player, _ui: slint::Weak<AppWindow>) -> (i8, u128, u128) {
    let ui_2: AppWindow = _ui.unwrap();
    let mut board = board::new_board();
    let mut players_turn = starting_player();
    let mut d;
    let mut d1 = 0;
    let mut d2 = 0;
    let mut d1_max = 0;
    let mut d2_max = 0;
    let mut ub;
    println!("starting player is {}\n", players_turn);
    loop {
        let m;
        let start = Instant::now();
        if players_turn == 1 {
            println!("player 1:");
            m = p1.play(&board);
            d = start.elapsed();
            d1 += d.as_millis();
            d1_max = d1_max.max(d.as_millis());
        } else {
            println!("player 2:");
            m = p2.play(&board);
            d = start.elapsed();
            d2 += d.as_millis();
            d2_max = d2_max.max(d.as_millis());
        }
        println!("Time is: {:?}", d);
        ub = board::update_board(&mut board, m, players_turn);
        ui::update_ui_board(board.clone(), ui_2.as_weak());
        if players_turn == -1 {
            println!();
            // if ub == 0 {board::print_board(&board)}
            // println!();
        }
        if ub != 0 {
            break;
        }
        players_turn *= -1;
    }
    println!("players:");
    println!("Time 1: {:?}s, Time 2: {:?}s", d1 / 1000, d2 / 1000);
    println!("ub: {}, d1_max: {}ms, d2_max: {}ms", ub, d1_max, d2_max);
    (ub, d1_max, d2_max)
}

fn _run(p1: player::Player, p2: player::Player) -> (i8, u128, u128) {
    let mut board = board::new_board();
    let mut players_turn = starting_player();
    let mut d;
    let mut d1 = 0;
    let mut d2 = 0;
    let mut d1_max = 0;
    let mut d2_max = 0;
    let mut ub;
    println!("starting player is {}\n", players_turn);
    loop {
        let m;
        let start = Instant::now();
        if players_turn == 1 {
            println!("player 1:");
            m = p1.play(&board);
            d = start.elapsed();
            d1 += d.as_millis();
            d1_max = d1_max.max(d.as_millis());
        } else {
            println!("player 2:");
            m = p2.play(&board);
            d = start.elapsed();
            d2 += d.as_millis();
            d2_max = d2_max.max(d.as_millis());
        }
        println!("Time is: {:?}", d);
        ub = board::update_board(&mut board, m, players_turn);
        if players_turn == -1 {
            println!();
            // if ub == 0 {board::print_board(&board)}
            // println!();
        }
        if ub != 0 {
            break;
        }
        players_turn *= -1;
    }
    println!("players:");
    println!("Time 1: {:?}s, Time 2: {:?}s", d1 / 1000, d2 / 1000);
    println!("ub: {}, d1_max: {}ms, d2_max: {}ms", ub, d1_max, d2_max);
    (ub, d1_max, d2_max)
}


pub fn _setup_game() {
    let p1 = player::_select_player( 1);
    let p2 = player::_select_player(-1);
    _run(p1, p2);
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_random() {
        let p1 = player::_select_player_in_code(1, 0);
        let p2 = player::_select_player_in_code(-1, 0);
        let result = _run(p1, p2);
        assert_ne!(result.0, 0);
        assert_ne!(result.0.abs(), 2);
    }


    #[test]
    fn test_run_random_vs_random_10000() {
        let mut p1_wins = 0;
        let mut p2_wins = 0;
        let mut draws = 0;
        for _ in 0..10000 {
            let p1 = player::_select_player_in_code(1, 0);
            let p2 = player::_select_player_in_code(-1, 0);
            let result = _run(p1, p2);
            assert_ne!(result.0, 0);
            assert_ne!(result.0.abs(), 2);
            if result.0 == 3 {
                draws += 1;
            } else if result.0 == 1 {
                p1_wins += 1;
            } else if result.0 == -1 {
                p2_wins += 1;
            }
        }
        println!("\np1w: {} p2w: {} d: {}", p1_wins, p2_wins, draws);
        assert!(p1_wins > ((10000-draws) *4) /10);
        assert!(p2_wins > ((10000-draws) *4) /10);
    }

    // #[test]
    fn _test_run_2_vs_2() {
        let mut p1_wins = 0;
        let mut p2_wins = 0;
        let mut draws = 0;
        let cycles = 100;
        for _ in 0..cycles {
            let p1 = player::_select_player_in_code(1, 2);
            let p2 = player::_select_player_in_code(-1, 2);
            let result = _run(p1, p2);
            println!("results = {:?}", result);
            assert_ne!(result.0, 0);
            assert_ne!(result.0.abs(), 2);
            if result.0 == 3 {
                draws += 1;
            } else if result.0 == 1 {
                p1_wins += 1;
            } else if result.0 == -1 {
                p2_wins += 1;
            }
        }
        println!("\np1w: {} p2w: {} d: {}", p1_wins, p2_wins, draws);
        assert!(p1_wins > ((cycles-draws) *4) /10);
        assert!(p2_wins > ((cycles-draws) *4) /10);
    }

    #[test]
    fn _test_run_3_vs_3() {
        let mut p1_wins = 0;
        let mut p2_wins = 0;
        let mut draws = 0;
        let cycles = 100;
        for _ in 0..cycles {
            let p1 = player::_select_player_in_code(1, 3);
            let p2 = player::_select_player_in_code(-1, 3);
            let result = _run(p1, p2);
            println!("results = {:?}", result);
            assert_ne!(result.0, 0);
            assert_ne!(result.0.abs(), 2);
            if result.0 == 3 {
                draws += 1;
            } else if result.0 == 1 {
                p1_wins += 1;
            } else if result.0 == -1 {
                p2_wins += 1;
            }
        }
        println!("\np1w: {} p2w: {} d: {}", p1_wins, p2_wins, draws);
        assert!(p1_wins > ((cycles-draws) *4) /10);
        assert!(p2_wins > ((cycles-draws) *4) /10);
    }

    // #[test]
    fn _test_run_4_vs_4() {
        let mut p1_wins = 0;
        let mut p2_wins = 0;
        let mut draws = 0;
        let cycles = 100;
        for _ in 0..cycles {
            let p1 = player::_select_player_in_code(1, 4);
            let p2 = player::_select_player_in_code(-1, 4);
            let result = _run(p1, p2);
            println!("results = {:?}", result);
            assert_ne!(result.0, 0);
            assert_ne!(result.0.abs(), 2);
            if result.0 == 3 {
                draws += 1;
            } else if result.0 == 1 {
                p1_wins += 1;
            } else if result.0 == -1 {
                p2_wins += 1;
            }
        }
        println!("\np1w: {} p2w: {} d: {}", p1_wins, p2_wins, draws);
        assert!(p1_wins > ((cycles-draws) *4) /10);
        assert!(p2_wins > ((cycles-draws) *4) /10);
    }

    // #[test]
    fn _test_run_2_vs_3() {       // 40-60-0
        let mut p1_wins = 0;
        let mut p2_wins = 0;
        let mut draws = 0;
        let cycles = 100;
        for _ in 0..cycles {
            let p1 = player::_select_player_in_code(1, 2);
            let p2 = player::_select_player_in_code(-1, 3);
            let result = _run(p1, p2);
            println!("results = {:?}", result);
            assert_ne!(result.0, 0);
            assert_ne!(result.0.abs(), 2);
            if result.0 == 3 {
                draws += 1;
            } else if result.0 == 1 {
                p1_wins += 1;
            } else if result.0 == -1 {
                p2_wins += 1;
            }
        }
        println!("\np1w: {} p2w: {} d: {}", p1_wins, p2_wins, draws);
        assert!(false);
    }

    // #[test]
    fn _test_run_2_vs_4() {       // 99-1-0
        let mut p1_wins = 0;
        let mut p2_wins = 0;
        let mut draws = 0;
        let cycles = 100;
        for _ in 0..cycles {
            let p1 = player::_select_player_in_code(1, 2);
            let p2 = player::_select_player_in_code(-1, 4);
            let result = _run(p1, p2);
            println!("results = {:?}", result);
            assert_ne!(result.0, 0);
            assert_ne!(result.0.abs(), 2);
            if result.0 == 3 {
                draws += 1;
            } else if result.0 == 1 {
                p1_wins += 1;
            } else if result.0 == -1 {
                p2_wins += 1;
            }
        }
        println!("\np1w: {} p2w: {} d: {}", p1_wins, p2_wins, draws);
        assert!(false);
    }

    // #[test]
    fn _test_run_3_vs_4() {       // 100-0-0
        let mut p1_wins = 0;
        let mut p2_wins = 0;
        let mut draws = 0;
        let cycles = 100;
        for _ in 0..cycles {
            let p1 = player::_select_player_in_code(1, 3);
            let p2 = player::_select_player_in_code(-1, 4);
            let result = _run(p1, p2);
            println!("results = {:?}", result);
            assert_ne!(result.0, 0);
            assert_ne!(result.0.abs(), 2);
            if result.0 == 3 {
                draws += 1;
            } else if result.0 == 1 {
                p1_wins += 1;
            } else if result.0 == -1 {
                p2_wins += 1;
            }
        }
        println!("\np1w: {} p2w: {} d: {}", p1_wins, p2_wins, draws);
        assert!(false);
    }
}