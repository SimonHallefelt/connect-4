

use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Instant;
use rand::Rng;

use crate::player;
use crate::board;


pub struct Game {
    p1: player::Player,
    p2: player::Player,
    board: Vec<Vec<i8>>,
    running: bool,
    potential_move: i8,
    // players_turn: i8,
    // d1: u128,
    // d2: u128,
    // d1_max: u128,
    // d2_max: u128,
    // ub: i8,
}

impl Game {
    pub fn new_game() -> Game {
        Game {
            p1: player::_select_player_in_code( 1, 0),
            p2: player::_select_player_in_code(-1, 0),
            board: board::new_board(),
            running: false,
            potential_move: -1,
        }
    }

    pub fn update_player(&mut self, player: i8, player_type: i8) {
        if player == 1 {
            self.p1 = player::_select_player_in_code(player, player_type);
        } else {
            self.p2 = player::_select_player_in_code(player, player_type);
        }
    }

    pub fn update_potential_move(&mut self, potential_move: i8) {
        self.potential_move = potential_move;
    }

    fn update_board(&mut self, board_2: Vec<Vec<i8>>) {
        self.board = board_2;
    }

    pub fn get_board(&self) -> Vec<Vec<i8>> {
        self.board.clone()
    }

    fn update_running(&mut self, running: bool) {
        self.running = running;
    }

    pub fn get_running(&self) -> bool {
        self.running
    }
}

pub fn game_run(game: Arc<Mutex<Game>>) {
    let g = Arc::clone(&game);
    let mut game = g.lock().unwrap();
    if game.running {
        return;
    }
    game.update_running(true);
    drop(game);
    thread::spawn(move || {
        run(g);
    });
}

fn starting_player() -> i8 {
    let mut rng = rand::thread_rng();
    match rng.gen_range(0..2) {
        1 => return 1,
        _ => return -1,
    }
}

fn run(g: Arc<Mutex<Game>>) -> (i8, u128, u128) {
    let mut board = board::new_board();
    let mut players_turn = starting_player();
    let mut d;
    let mut d1 = 0;
    let mut d2 = 0;
    let mut d1_max = 0;
    let mut d2_max = 0;
    let mut ub;
    let mut game = g.lock().unwrap();
    let p1 = game.p1.clone();
    let p2 = game.p2.clone();
    game.update_board(board.clone());
    drop(game);
    println!("starting player is {}\n", players_turn);
    loop {
        let m;
        let start = Instant::now();
        if players_turn == 1 {
            println!("player 1:");
            m = player_move(&board, &p1, g.clone());
            d = start.elapsed();
            d1 += d.as_millis();
            d1_max = d1_max.max(d.as_millis());
        } else {
            println!("player 2:");
            m = player_move(&board, &p2, g.clone());
            d = start.elapsed();
            d2 += d.as_millis();
            d2_max = d2_max.max(d.as_millis());
        }
        println!("Time is: {:?}", d);
        ub = board::update_board(&mut board, m, players_turn);
        let mut game = g.lock().unwrap();
        game.update_board(board.clone());
        if players_turn == -1 {
            println!();
            if ub == 0 {board::print_board(&board)}
            println!();
        }
        if ub != 0 {
            game.update_running(false);
            break;
        }
        players_turn *= -1;
    }
    println!("players:");
    println!("Time 1: {:?}s, Time 2: {:?}s", d1 / 1000, d2 / 1000);
    println!("ub: {}, d1_max: {}ms, d2_max: {}ms", ub, d1_max, d2_max);
    (ub, d1_max, d2_max)
}


fn player_move(board: &Vec<Vec<i8>>, player: &player::Player, game: Arc<Mutex<Game>>) -> i8 {
    if player.get_player_type() == 5 {
        loop {
            thread::sleep(std::time::Duration::from_millis(10));
            let mut g = game.lock().unwrap();
            if g.potential_move == -1 {
                continue;
            }
            let play = player.play(&board, g.potential_move);
            if play != -1 {
                g.update_potential_move(-1);
                return play;
            }
        }
    }
    player.play(&board, -1)
}



pub fn _setup_game() {
    let p1 = player::_select_player( 1);
    let p2 = player::_select_player(-1);
    let mut g = Game::new_game();
    g.p1 = p1;
    g.p2 = p2;
    run(Arc::new(Mutex::new(g)));
}



#[cfg(test)]
mod tests {
    use super::*;

    fn make_game(p1: i8, p2: i8) -> Arc<Mutex<Game>> {
        let mut g = Game::new_game();
        g.p1 = player::_select_player_in_code( 1, p1);
        g.p2 = player::_select_player_in_code(-1, p2);
        Arc::new(Mutex::new(g))
    }

    #[test]
    fn test_run_random() {
        let g = make_game(0, 0);
        let result = run(g);
        assert_ne!(result.0, 0);
        assert_ne!(result.0.abs(), 2);
    }


    #[test]
    fn test_run_random_vs_random_10000() {
        let mut p1_wins = 0;
        let mut p2_wins = 0;
        let mut draws = 0;
        for _ in 0..10000 {
            let g = make_game(0, 0);
            let result = run(g);
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
    fn _test_run_1_vs_1() {
        let mut p1_wins = 0;
        let mut p2_wins = 0;
        let mut draws = 0;
        let cycles = 100;
        for _ in 0..cycles {
            let g = make_game(1, 1);
            let result = run(g);
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
    fn _test_run_2_vs_2() {
        let mut p1_wins = 0;
        let mut p2_wins = 0;
        let mut draws = 0;
        let cycles = 100;
        for _ in 0..cycles {
            let g = make_game(2, 2);
            let result = run(g);
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
    fn _test_run_3_vs_3() {
        let mut p1_wins = 0;
        let mut p2_wins = 0;
        let mut draws = 0;
        let cycles = 100;
        for _ in 0..cycles {
            let g = make_game(3, 3);
            let result = run(g);
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
    fn _test_run_1_vs_2() {       // 40-60-0
        let mut p1_wins = 0;
        let mut p2_wins = 0;
        let mut draws = 0;
        let cycles = 100;
        for _ in 0..cycles {
            let g = make_game(1, 2);
            let result = run(g);
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
    fn _test_run_1_vs_3() {       // 99-1-0
        let mut p1_wins = 0;
        let mut p2_wins = 0;
        let mut draws = 0;
        let cycles = 100;
        for _ in 0..cycles {
            let g = make_game(1, 3);
            let result = run(g);
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
    fn _test_run_2_vs_3() {       // 100-0-0
        let mut p1_wins = 0;
        let mut p2_wins = 0;
        let mut draws = 0;
        let cycles = 100;
        for _ in 0..cycles {
            let g = make_game(2, 3);
            let result = run(g);
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