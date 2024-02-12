use rand::Rng;

mod board;

fn starting_player() -> i8 {
    let mut rng = rand::thread_rng();
    match rng.gen_range(0..2) {
        1 => return 1,
        _ => return -1,
    }
}

fn main() {
    let mut board = board::new_board();
    let mut player = starting_player();
    let mut rng = rand::thread_rng();
    loop {
        let m = rng.gen_range(0..7);
        board::update_board(&mut board, m, player);
        player *= -1;
    }
}

