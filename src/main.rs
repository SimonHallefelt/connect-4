use rand::Rng;
use std::time::Instant;

mod board;
mod player;

fn starting_player() -> i8 {
    let mut rng = rand::thread_rng();
    match rng.gen_range(0..2) {
        1 => return 1,
        _ => return -1,
    }
}


fn run(p1: player::Player, p2: player::Player) -> (i8, u128, u128) {
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
            if ub == 0 {board::print_board(&board)}
            println!("player total time (seconds)");
            println!("Time 1: {:?} Time 2: {:?}", d1 / 1000, d2 / 1000);
            println!();
        }
        if ub != 0 {
            break;
        }
        players_turn *= -1;
    }
    (ub, d1_max, d2_max)
}


fn select_player() {
    let p1 = player::select_player( 1);
    let p2 = player::select_player(-1);
    run(p1, p2);
}


fn main() {
    select_player();
}




#[cfg(test)]
mod tests {
    use std::result;

    use super::*;

    #[test]
    fn test_run_random() {
        let p1 = player::_select_player_in_code(1, 0);
        let p2 = player::_select_player_in_code(-1, 0);
        let result = run(p1, p2);
        assert_ne!(result.0, 0);
        assert_ne!(result.0.abs(), 2);
        assert!(result.1 < 5000);
        assert!(result.2 < 5000);
    }


    #[test]
    fn test_run_random_vs_random_10000() {
        let mut p1_wins = 0;
        let mut p2_wins = 0;
        let mut draws = 0;
        for _ in 0..10000 {
            let p1 = player::_select_player_in_code(1, 0);
            let p2 = player::_select_player_in_code(-1, 0);
            let result = run(p1, p2);
            assert_ne!(result.0, 0);
            assert_ne!(result.0.abs(), 2);
            assert!(result.1 < 5000);
            assert!(result.2 < 5000);
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

    #[test]
    fn _test_run_2_vs_2() {
        let mut p1_wins = 0;
        let mut p2_wins = 0;
        let mut draws = 0;
        let cycles = 100;
        for _ in 0..cycles {
            let p1 = player::_select_player_in_code(1, 2);
            let p2 = player::_select_player_in_code(-1, 2);
            let result = run(p1, p2);
            println!("results = {:?}", result);
            assert_ne!(result.0, 0);
            assert_ne!(result.0.abs(), 2);
            assert!(result.1 < 5000);
            assert!(result.2 < 5000);
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
            let result = run(p1, p2);
            println!("results = {:?}", result);
            assert_ne!(result.0, 0);
            assert_ne!(result.0.abs(), 2);
            assert!(result.1 < 5000);
            assert!(result.2 < 5000);
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
    fn _test_run_4_vs_4() {
        let mut p1_wins = 0;
        let mut p2_wins = 0;
        let mut draws = 0;
        let cycles = 100;
        for _ in 0..cycles {
            let p1 = player::_select_player_in_code(1, 4);
            let p2 = player::_select_player_in_code(-1, 4);
            let result = run(p1, p2);
            println!("results = {:?}", result);
            assert_ne!(result.0, 0);
            assert_ne!(result.0.abs(), 2);
            assert!(result.1 < 5000);
            assert!(result.2 < 5000);
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
    fn _test_run_2_vs_3() {       // 
        let mut p1_wins = 0;
        let mut p2_wins = 0;
        let mut draws = 0;
        let cycles = 100;
        for _ in 0..cycles {
            let p1 = player::_select_player_in_code(1, 2);
            let p2 = player::_select_player_in_code(-1, 3);
            let result = run(p1, p2);
            println!("results = {:?}", result);
            assert_ne!(result.0, 0);
            assert_ne!(result.0.abs(), 2);
            assert!(result.1 < 5000);
            assert!(result.2 < 5000);
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

    #[test]
    fn _test_run_2_vs_4() {       // 
        let mut p1_wins = 0;
        let mut p2_wins = 0;
        let mut draws = 0;
        let cycles = 100;
        for _ in 0..cycles {
            let p1 = player::_select_player_in_code(1, 2);
            let p2 = player::_select_player_in_code(-1, 4);
            let result = run(p1, p2);
            println!("results = {:?}", result);
            assert_ne!(result.0, 0);
            assert_ne!(result.0.abs(), 2);
            assert!(result.1 < 5000);
            assert!(result.2 < 5000);
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

    #[test]
    fn _test_run_3_vs_4() {       //
        let mut p1_wins = 0;
        let mut p2_wins = 0;
        let mut draws = 0;
        let cycles = 100;
        for _ in 0..cycles {
            let p1 = player::_select_player_in_code(1, 3);
            let p2 = player::_select_player_in_code(-1, 4);
            let result = run(p1, p2);
            println!("results = {:?}", result);
            assert_ne!(result.0, 0);
            assert_ne!(result.0.abs(), 2);
            assert!(result.1 < 5000);
            assert!(result.2 < 5000);
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