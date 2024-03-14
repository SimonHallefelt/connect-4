

pub fn get_move(board: &Vec<Vec<i8>>, potential_move: i8) -> i8 {
    let legal_moves = get_legal_moves(board);
    if !legal_moves.contains(&potential_move.to_string()) {
        return -1;
    }
    return potential_move;
}


fn get_legal_moves(board: &Vec<Vec<i8>>) -> Vec<String> {
    let mut legal_moves = Vec::new();
    for i in 0..7 {
        if board[0][i as usize] == 0 {
            legal_moves.push(i.to_string());
        }
    }
    legal_moves
}


fn _print_board(board: &Vec<Vec<i8>>) {
    println!("board:");
    for b in board {
        print!("[");
        for n in b {
            if *n == 1 {print!("  1 ")}
            else if *n == -1 {print!(" -1 ")}
            else {print!("  0 ")}
        }
        println!("]");
    }
}