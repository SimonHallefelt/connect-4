use rand::Rng;

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
    println!("starting player is {}", players_turn);
    loop {
        let m;
        if players_turn == 1 {
            m = p1.play(&board);
        } else {
            m = p2.play(&board);
        }
        board::update_board(&mut board, m, players_turn);
        if players_turn == 1 {
            board::print_board(&board);
        }
        players_turn *= -1;
    }
}

