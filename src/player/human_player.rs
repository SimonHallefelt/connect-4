

pub fn get_move(board: &Vec<Vec<i8>>) -> i8 {
    let legal_moves = get_legal_moves(board);
    print_board(board);
    let mut m: String = String::new();
    println!("enter your move: ");
    while !legal_moves.contains(&m) {
        m = String::new();
        print!("enter your move: ");
        std::io::stdin().read_line(&mut m).unwrap();
        m = m.trim().to_string();
        println!()
    }
    return m.parse::<i8>().unwrap();
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


fn print_board(board: &Vec<Vec<i8>>) {
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