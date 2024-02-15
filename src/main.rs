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

fn main() {
    let mut board = board::new_board();
    let mut players_turn = starting_player();
    let p1 = player::select_player( 1);
    let p2 = player::select_player(-1);
    let mut d;
    let mut d1 = 0;
    let mut d2 = 0;
    println!("starting player is {}\n", players_turn);
    loop {
        let m;
        let start = Instant::now();
        if players_turn == 1 {
            println!("player 1:");
            m = p1.play(&board);
            d = start.elapsed();
            d1 += d.as_millis();
        } else {
            println!("player 2:");
            m = p2.play(&board);
            d = start.elapsed();
            d2 += d.as_millis();
        }
        println!("Time is: {:?}", d);
        board::update_board(&mut board, m, players_turn);
        if players_turn == -1 {
            println!();
            board::print_board(&board);
            println!("player total time (seconds)");
            println!("Time 1: {:?} Time 2: {:?}", d1 / 1000, d2 / 1000);
            println!();
        }
        players_turn *= -1;
    }
}

