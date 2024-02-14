use std::vec;

use rand::seq::SliceRandom;


pub fn get_move(board: &Vec<Vec<i8>>, player: i8) -> i8{
    let depth = 10;
    let mut bitboard: Vec<i64> = Vec::with_capacity(2);
    let mut moves: Vec<i32> = Vec::with_capacity(42);
    for _ in 0..42 {
        moves.push(0)
    }
    let mut height: Vec<i32> = vec![0, 7, 14, 21, 28, 35, 42];

    print_board(board);
    if player == 1 {
        make_bitboards(board, &mut bitboard, &mut height);
        let m = start_alpha_beta(player, depth, &mut height, &mut bitboard, &mut moves);
        return m;
    }else {
        let new_board = flip(board);
        make_bitboards(&new_board, &mut bitboard, &mut height);
        let m = start_alpha_beta(player*-1, depth, &mut height, &mut bitboard, &mut moves);
        return m;
    }

}


fn start_alpha_beta(player: i8, depth: i8, height: &mut Vec<i32>, bitboard: &mut Vec<i64>, moves: &mut Vec<i32>) -> i8 {
    let mut best_move = -1;
    let mut alpha = -10000000;
    let beta = 10000000;
    println!("starting bitboards"); print_bitboards(bitboard); println!("--");

    for lm in list_moves(&height) {
        let score = alpha_beta(player*-1, depth-1, lm, 0, alpha, beta, height, bitboard, moves, 0);
        if score > alpha {
            alpha = score;
            best_move = lm;
        }
        if alpha == 1000 {
            break;
        }
    }
    best_move
}


fn alpha_beta(player: i8, depth: i8, m: i8, old_score: i32, mut alpha: i32, mut beta: i32, height: &mut Vec<i32>, bitboard: &mut Vec<i64>, moves: &mut Vec<i32>, mut counter: usize) -> i32 {
    make_move(m as usize, height, bitboard, moves, counter);
    let mut score = calc_score(bitboard, counter);
    counter += 1;
    if score == 1000 {
        if player == -1 {
            undo_move(height, bitboard, moves, counter);
            return score;
        } else {
            undo_move(height, bitboard, moves, counter);
            return (score - (100 - depth as i32)) * -1;
        }
    }
    score = (score * player as i32 * -1) + old_score;
    if depth == 0 {
        undo_move(height, bitboard, moves, counter);
        return score;
    }
    for lm in list_moves(&height) {
        let new_score = alpha_beta(player*-1, depth-1, lm, score, alpha, beta, height, bitboard, moves, counter);
        if player == 1 {
            if new_score > alpha {
                alpha = new_score;
                if alpha >= beta || alpha == 1000 {
                    undo_move(height, bitboard, moves, counter);
                    return alpha;
                }
            }
        }else {
            if new_score < beta {
                beta = new_score;
                if alpha >= beta || beta < -800 {
                    undo_move(height, bitboard, moves, counter);
                    return beta;
                }
            }
        }
    }
    undo_move(height, bitboard, moves, counter);
    if player == 1 {
        return alpha;
    } else {
        return beta;
    }
}


fn calc_score(bitboard: &Vec<i64>, counter: usize) -> i32 {
    let s = [0, 1, 2, 3, 4, 5, 6, 7];
    let mut score = *s.choose(&mut rand::thread_rng()).unwrap();
    
    if counter & 1 == 0 {
        if is_win(bitboard[0]) {
            score = 1000;
        }
    } else {
        if is_win(bitboard[1]) {
            score = 1000;
        }
    }
    
    score
}



fn list_moves(height: &Vec<i32>) -> Vec<i8> {
    let mut legal_moves = Vec::new();
    let top: i64 = 0b1000000_1000000_1000000_1000000_1000000_1000000_1000000;
    for i in 0..7 {
        if (top & (1 << height[i])) == 0 {
            legal_moves.push(i as i8);
        }
    }
    legal_moves
}


fn make_bitboards(board: &Vec<Vec<i8>>, bitboard: &mut Vec<i64>, height: &mut Vec<i32>) {
    bitboard.push(0);
    bitboard.push(0);
    for i in (0..6).rev() {
        for j in 0..7 {
            if board[i][j] == 1 {
                make_initial_move(j, height, bitboard, 0)
            }else if board[i][j] == -1 {
                make_initial_move(j, height, bitboard, 1)
            }
        }
    }
}


fn is_win(bitboard: i64) -> bool {
    let directions = [1, 7, 6, 8];
    let mut bb;
    for d in directions {
        bb = bitboard & (bitboard >> d);
        if bb & (bb >> (d*2)) != 0 {
            return true;
        }
    }
    false
}


fn undo_move(height: &mut Vec<i32>, bitboard: &mut Vec<i64>, moves: &mut Vec<i32>, mut counter: usize) {
    counter -= 1;
    let col = moves[counter] as usize;
    height[col] -= 1;
    let movee = 1 << height[col];
    bitboard[counter & 1] ^= movee;
}


fn make_move(col: usize, height: &mut Vec<i32>, bitboard: &mut Vec<i64>, moves: &mut Vec<i32>, counter: usize) {
    let movee = 1 << height[col];
    height[col] += 1;
    bitboard[counter & 1] ^= movee;
    moves[counter] = col as i32
    // counter += 1;
}


fn make_initial_move(col: usize, height: &mut Vec<i32>, bitboard: &mut Vec<i64>, player: usize) {
    let movee = 1 << height[col];
    height[col] += 1;
    bitboard[player] ^= movee;
}


fn flip(board: &Vec<Vec<i8>>) -> Vec<Vec<i8>> {
    let mut new_board = Vec::new();
    for i in 0..6 {
        new_board.push(Vec::new());
        for j in 0..7 {
            new_board[i].push(board[i][j]*-1);
        }
    }
    new_board
}


fn print_bitboards(bitboard: &Vec<i64>) {
    for i in 0..2 {
        let mask = 0b0000000_0000000_0000000_0000000_0000000_0000000_0000001;
        for j in (0..6).rev() {     // 0..7 for full board
            for k in 0..7 {         // 0..9 for full board
                if (mask & (bitboard[i] >> (j + k*7))) != 0 {
                    print!("1 ");
                }else {
                    print!("0 ");
                }
            }
            println!();
        }
        println!();
    }
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




#[cfg(test)]
mod tests {
    use super::*;

    fn new_board() -> Vec<Vec<i8>> {
        let mut board = Vec::new();
        for i in 0..6 {
            board.push(Vec::new());
            for _ in 0..7 {
                board[i].push(0);
            }
        }
        board
    }

    fn get_height() -> Vec<i32> {
        return vec![0, 7, 14, 21, 28, 35, 42];
    }

    fn get_moves() -> Vec<i32> {
        let mut moves: Vec<i32> = Vec::with_capacity(42);
        for _ in 0..42 {
            moves.push(0)
        }
        moves
    }

    #[test]
    fn test_new_board_test() {
        let board = new_board();
        assert_eq!(board.len(), 6);
        assert_eq!(board[0].len(), 7);
    }


    #[test]
    fn test_make_initial_move_1() {
        let mut bitboards = vec![0,0];
        let mut height = get_height();
        make_initial_move(0, &mut height, &mut bitboards, 0);
        print_bitboards(&bitboards);
        assert_eq!(height, vec![1, 7, 14, 21, 28, 35, 42]);
        assert_eq!(bitboards[0], 0b0000000_0000000_0000000_0000000_0000000_0000000_0000001);
        assert_eq!(bitboards[1], 0b0000000_0000000_0000000_0000000_0000000_0000000_0000000);
        make_initial_move(4, &mut height, &mut bitboards, 1);
        print_bitboards(&bitboards);
        assert_eq!(height, vec![1, 7, 14, 21, 29, 35, 42]);
        assert_eq!(bitboards[0], 0b0000000_0000000_0000000_0000000_0000000_0000000_0000001);
        assert_eq!(bitboards[1], 0b0000000_0000000_0000001_0000000_0000000_0000000_0000000);
    }


    #[test]
    fn test_make_initial_move_2() {
        let mut bitboards = vec![0,0];
        let mut height = get_height();
        make_initial_move(0, &mut height, &mut bitboards, 0);
        make_initial_move(3, &mut height, &mut bitboards, 1);
        make_initial_move(0, &mut height, &mut bitboards, 0);
        print_bitboards(&bitboards);
        assert_eq!(height, vec![2, 7, 14, 22, 28, 35, 42]);
        assert_eq!(bitboards[0], 0b0000000_0000000_0000000_0000000_0000000_0000000_0000011);
        assert_eq!(bitboards[1], 0b0000000_0000000_0000000_0000001_0000000_0000000_0000000);
        make_initial_move(4, &mut height, &mut bitboards, 1);
        make_initial_move(4, &mut height, &mut bitboards, 0);
        print_bitboards(&bitboards);
        assert_eq!(height, vec![2, 7, 14, 22, 30, 35, 42]);
        assert_eq!(bitboards[0], 0b0000000_0000000_0000010_0000000_0000000_0000000_0000011);
        assert_eq!(bitboards[1], 0b0000000_0000000_0000001_0000001_0000000_0000000_0000000);
    }


    #[test]
    fn test_make_bitboards_1() {
        let mut board = new_board();
        board[5] = vec![0, 1, -1, 0, 1, -1, 0];
        let mut bitboards = Vec::new();
        let mut height = get_height();
        make_bitboards(&mut board, &mut bitboards, &mut height);
        print_bitboards(&bitboards);
        assert_eq!(height, vec![0, 8, 15, 21, 29, 36, 42]);
        assert_eq!(bitboards[0], 0b0000000_0000000_0000001_0000000_0000000_0000001_0000000);
        assert_eq!(bitboards[1], 0b0000000_0000001_0000000_0000000_0000001_0000000_0000000);
    }


    #[test]
    fn test_make_bitboards_2() {
        let mut board = new_board();
        board[5] = vec![0,  1, -1, 0,  1, -1, 0];
        board[4] = vec![0, -1,  1, 0, -1,  0, 0];
        let mut bitboards = Vec::new();
        let mut height = get_height();
        make_bitboards(&mut board, &mut bitboards, &mut height);
        print_bitboards(&bitboards);
        assert_eq!(height, vec![0, 9, 16, 21, 30, 36, 42]);
        assert_eq!(bitboards[0], 0b0000000_0000000_0000001_0000000_0000010_0000001_0000000);
        assert_eq!(bitboards[1], 0b0000000_0000001_0000010_0000000_0000001_0000010_0000000);
    }


    #[test]
    fn test_make_move_1() {
        let mut board = new_board();
        let mut bitboards = Vec::new();
        let mut height = get_height();
        let mut moves = get_moves();
        let mut counter = 0;
        make_bitboards(&mut board, &mut bitboards, &mut height);
        make_move(0, &mut height, &mut bitboards, &mut moves, counter);
        counter += 1;
        print_bitboards(&bitboards);
        assert_eq!(height, vec![1, 7, 14, 21, 28, 35, 42]);
        assert_eq!(moves[0..1], vec![0]);
        assert_eq!(bitboards[0], 0b0000000_0000000_0000000_0000000_0000000_0000000_0000001);
        assert_eq!(bitboards[1], 0b0000000_0000000_0000000_0000000_0000000_0000000_0000000);
        make_move(5, &mut height, &mut bitboards, &mut moves, counter);
        print_bitboards(&bitboards);
        assert_eq!(height, vec![1, 7, 14, 21, 28, 36, 42]);
        assert_eq!(moves[0..2], vec![0, 5]);
        assert_eq!(bitboards[0], 0b0000000_0000000_0000000_0000000_0000000_0000000_0000001);
        assert_eq!(bitboards[1], 0b0000000_0000001_0000000_0000000_0000000_0000000_0000000);
    }


    #[test]
    fn test_make_move_2() {
        let mut board = new_board();
        let mut bitboards = Vec::new();
        let mut height = get_height();
        let mut moves = get_moves();
        let mut counter = 0;
        make_bitboards(&mut board, &mut bitboards, &mut height);
        make_move(0, &mut height, &mut bitboards, &mut moves, counter);
        counter += 1;
        make_move(5, &mut height, &mut bitboards, &mut moves, counter);
        counter += 1;
        make_move(4, &mut height, &mut bitboards, &mut moves, counter);
        counter += 1;
        print_bitboards(&bitboards);
        assert_eq!(height, vec![1, 7, 14, 21, 29, 36, 42]);
        assert_eq!(moves[0..3], vec![0, 5, 4]);
        assert_eq!(bitboards[0], 0b0000000_0000000_0000001_0000000_0000000_0000000_0000001);
        assert_eq!(bitboards[1], 0b0000000_0000001_0000000_0000000_0000000_0000000_0000000);
        make_move(4, &mut height, &mut bitboards, &mut moves, counter);
        counter += 1;
        make_move(5, &mut height, &mut bitboards, &mut moves, counter);
        print_bitboards(&bitboards);
        assert_eq!(height, vec![1, 7, 14, 21, 30, 37, 42]);
        assert_eq!(moves[0..5], vec![0, 5, 4, 4, 5]);
        assert_eq!(bitboards[0], 0b0000000_0000010_0000001_0000000_0000000_0000000_0000001);
        assert_eq!(bitboards[1], 0b0000000_0000001_0000010_0000000_0000000_0000000_0000000);
    }


    #[test]
    fn test_undo_move_1() {
        let mut board = new_board();
        let mut bitboards = Vec::new();
        board[5] = vec![0,  1, -1, 0,  1, 0, 0];
        let mut height = get_height();
        let mut moves = vec![4, 2, 1];
        let mut counter = moves.len();
        make_bitboards(&mut board, &mut bitboards, &mut height);
        undo_move(&mut height, &mut bitboards, &mut moves, counter); counter -= 1;
        print_bitboards(&bitboards);
        assert_eq!(height, vec![0, 7, 15, 21, 29, 35, 42]);
        assert_eq!(bitboards[0], 0b0000000_0000000_0000001_0000000_0000000_0000000_0000000);
        assert_eq!(bitboards[1], 0b0000000_0000000_0000000_0000000_0000001_0000000_0000000);
        
        undo_move(&mut height, &mut bitboards, &mut moves, counter);
        print_bitboards(&bitboards);
        assert_eq!(height, vec![0, 7, 14, 21, 29, 35, 42]);
        assert_eq!(bitboards[0], 0b0000000_0000000_0000001_0000000_0000000_0000000_0000000);
        assert_eq!(bitboards[1], 0b0000000_0000000_0000000_0000000_0000000_0000000_0000000);
    }


    #[test]
    fn test_undo_move_2() {
        let mut board = new_board();
        board[5] = vec![0,  1, -1, -1,  1, -1, 0];
        board[4] = vec![0,  -1, 1, 0,  1, 0, 0];
        let mut bitboards = Vec::new();
        let mut height = get_height();
        let mut moves = vec![4, 5, 4, 2, 2, 3, 1, 1];
        let mut counter = moves.len();
        make_bitboards(&mut board, &mut bitboards, &mut height);
        print_bitboards(&bitboards); println!("---");
        undo_move(&mut height, &mut bitboards, &mut moves, counter); counter -= 1;
        undo_move(&mut height, &mut bitboards, &mut moves, counter); counter -= 1;
        print_bitboards(&bitboards); println!("---");
        assert_eq!(height, vec![0, 7, 16, 22, 30, 36, 42]);
        assert_eq!(bitboards[0], 0b0000000_0000000_0000011_0000000_0000010_0000000_0000000);
        assert_eq!(bitboards[1], 0b0000000_0000001_0000000_0000001_0000001_0000000_0000000);
        undo_move(&mut height, &mut bitboards, &mut moves, counter); counter -= 1;
        undo_move(&mut height, &mut bitboards, &mut moves, counter); counter -= 1;
        undo_move(&mut height, &mut bitboards, &mut moves, counter); counter -= 1;
        undo_move(&mut height, &mut bitboards, &mut moves, counter);
        print_bitboards(&bitboards);
        assert_eq!(height, vec![0, 7, 14, 21, 29, 36, 42]);
        assert_eq!(bitboards[0], 0b0000000_0000000_0000001_0000000_0000000_0000000_0000000);
        assert_eq!(bitboards[1], 0b0000000_0000001_0000000_0000000_0000000_0000000_0000000);
    }


    #[test]
    fn test_undo_move_3() {
        let mut board = new_board();
        board[5] = vec![0,  0, 0, 0,  0, 0,  1];
        board[4] = vec![0,  0, 0, 0,  0, 0, -1];
        board[3] = vec![0,  0, 0, 0,  0, 0,  1];
        let mut bitboards = Vec::new();
        let mut height = get_height();
        let mut moves = vec![6,6,6];
        let mut counter = moves.len();
        make_bitboards(&mut board, &mut bitboards, &mut height);
        print_bitboards(&bitboards); println!("---");
        undo_move(&mut height, &mut bitboards, &mut moves, counter); counter -= 1;
        print_bitboards(&bitboards); println!("---");
        assert_eq!(height, vec![0, 7, 14, 21, 28, 35, 44]);
        assert_eq!(bitboards[0], 0b0000001_0000000_0000000_0000000_0000000_0000000_0000000);
        assert_eq!(bitboards[1], 0b0000010_0000000_0000000_0000000_0000000_0000000_0000000);
        undo_move(&mut height, &mut bitboards, &mut moves, counter); counter -= 1;
        undo_move(&mut height, &mut bitboards, &mut moves, counter);
        print_bitboards(&bitboards);
        assert_eq!(height, vec![0, 7, 14, 21, 28, 35, 42]);
        assert_eq!(bitboards[0], 0b0000000_0000000_0000000_0000000_0000000_0000000_0000000);
        assert_eq!(bitboards[1], 0b0000000_0000000_0000000_0000000_0000000_0000000_0000000);
    }


    #[test]
    fn test_make_and_undo_move_1() {
        let mut board = new_board();
        let mut bitboards = Vec::new();
        let mut height = get_height();
        let mut moves = get_moves();
        let mut counter = 0;
        make_bitboards(&mut board, &mut bitboards, &mut height);
        make_move(0, &mut height, &mut bitboards, &mut moves, counter);
        counter += 1;
        make_move(5, &mut height, &mut bitboards, &mut moves, counter);
        counter += 1;
        print_bitboards(&bitboards);
        assert_eq!(height, vec![1, 7, 14, 21, 28, 36, 42]);
        assert_eq!(moves[0..2], vec![0, 5]);
        assert_eq!(bitboards[0], 0b0000000_0000000_0000000_0000000_0000000_0000000_0000001);
        assert_eq!(bitboards[1], 0b0000000_0000001_0000000_0000000_0000000_0000000_0000000);
        undo_move(&mut height, &mut bitboards, &mut moves, counter); counter -= 1;
        undo_move(&mut height, &mut bitboards, &mut moves, counter);
        assert_eq!(height, vec![0, 7, 14, 21, 28, 35, 42]);
        assert_eq!(bitboards[0], 0b0000000_0000000_0000000_0000000_0000000_0000000_0000000);
        assert_eq!(bitboards[1], 0b0000000_0000000_0000000_0000000_0000000_0000000_0000000);
    }


    #[test]
    fn test_make_and_undo_move_2() {
        let mut board = new_board();
        let mut bitboards = Vec::new();
        let mut height = get_height();
        let mut moves = get_moves();
        let mut counter = 0;
        make_bitboards(&mut board, &mut bitboards, &mut height);
        make_move(0, &mut height, &mut bitboards, &mut moves, counter);
        counter += 1;
        make_move(5, &mut height, &mut bitboards, &mut moves, counter);
        counter += 1;
        make_move(4, &mut height, &mut bitboards, &mut moves, counter);
        counter += 1;
        make_move(4, &mut height, &mut bitboards, &mut moves, counter);
        counter += 1;
        make_move(5, &mut height, &mut bitboards, &mut moves, counter);
        counter += 1;
        print_bitboards(&bitboards);
        assert_eq!(height, vec![1, 7, 14, 21, 30, 37, 42]);
        assert_eq!(moves[0..5], vec![0, 5, 4, 4, 5]);
        assert_eq!(bitboards[0], 0b0000000_0000010_0000001_0000000_0000000_0000000_0000001);
        assert_eq!(bitboards[1], 0b0000000_0000001_0000010_0000000_0000000_0000000_0000000);
        undo_move(&mut height, &mut bitboards, &mut moves, counter); counter -= 1;
        undo_move(&mut height, &mut bitboards, &mut moves, counter); counter -= 1;
        undo_move(&mut height, &mut bitboards, &mut moves, counter); counter -= 1;
        undo_move(&mut height, &mut bitboards, &mut moves, counter); counter -= 1;
        undo_move(&mut height, &mut bitboards, &mut moves, counter);
        assert_eq!(height, vec![0, 7, 14, 21, 28, 35, 42]);
        assert_eq!(bitboards[0], 0b0000000_0000000_0000000_0000000_0000000_0000000_0000000);
        assert_eq!(bitboards[1], 0b0000000_0000000_0000000_0000000_0000000_0000000_0000000);
    }


    #[test]
    fn test_make_and_undo_move_3() {
        let mut board = new_board();
        let mut bitboards = Vec::new();
        let mut height = get_height();
        let mut moves = get_moves();
        let mut counter = 0;
        make_bitboards(&mut board, &mut bitboards, &mut height);
        make_move(0, &mut height, &mut bitboards, &mut moves, counter);
        counter += 1;
        make_move(5, &mut height, &mut bitboards, &mut moves, counter);
        counter += 1;
        make_move(4, &mut height, &mut bitboards, &mut moves, counter);
        counter += 1;
        make_move(4, &mut height, &mut bitboards, &mut moves, counter);
        counter += 1;
        make_move(5, &mut height, &mut bitboards, &mut moves, counter);
        counter += 1;
        print_bitboards(&bitboards);
        assert_eq!(height, vec![1, 7, 14, 21, 30, 37, 42]);
        assert_eq!(moves[0..5], vec![0, 5, 4, 4, 5]);
        assert_eq!(bitboards[0], 0b0000000_0000010_0000001_0000000_0000000_0000000_0000001);
        assert_eq!(bitboards[1], 0b0000000_0000001_0000010_0000000_0000000_0000000_0000000);
        undo_move(&mut height, &mut bitboards, &mut moves, counter); counter -= 1;
        undo_move(&mut height, &mut bitboards, &mut moves, counter); counter -= 1;
        undo_move(&mut height, &mut bitboards, &mut moves, counter); counter -= 1;
        undo_move(&mut height, &mut bitboards, &mut moves, counter); counter -= 1;
        undo_move(&mut height, &mut bitboards, &mut moves, counter); counter -= 1;
        assert_eq!(height, vec![0, 7, 14, 21, 28, 35, 42]);
        assert_eq!(bitboards[0], 0b0000000_0000000_0000000_0000000_0000000_0000000_0000000);
        assert_eq!(bitboards[1], 0b0000000_0000000_0000000_0000000_0000000_0000000_0000000);
        make_move(4, &mut height, &mut bitboards, &mut moves, counter);
        counter += 1;
        make_move(4, &mut height, &mut bitboards, &mut moves, counter);
        counter += 1;
        make_move(4, &mut height, &mut bitboards, &mut moves, counter);
        counter += 1;
        make_move(4, &mut height, &mut bitboards, &mut moves, counter);
        counter += 1;
        make_move(4, &mut height, &mut bitboards, &mut moves, counter);
        counter += 1;
        print_bitboards(&bitboards);
        assert_eq!(height, vec![0, 7, 14, 21, 33, 35, 42]);
        assert_eq!(moves[0..5], vec![4, 4, 4, 4, 4]);
        assert_eq!(bitboards[0], 0b0000000_0000000_0010101_0000000_0000000_0000000_0000000);
        assert_eq!(bitboards[1], 0b0000000_0000000_0001010_0000000_0000000_0000000_0000000);
        undo_move(&mut height, &mut bitboards, &mut moves, counter); counter -= 1;
        undo_move(&mut height, &mut bitboards, &mut moves, counter); counter -= 1;
        undo_move(&mut height, &mut bitboards, &mut moves, counter); counter -= 1;
        undo_move(&mut height, &mut bitboards, &mut moves, counter); counter -= 1;
        undo_move(&mut height, &mut bitboards, &mut moves, counter);
        assert_eq!(height, vec![0, 7, 14, 21, 28, 35, 42]);
        assert_eq!(bitboards[0], 0b0000000_0000000_0000000_0000000_0000000_0000000_0000000);
        assert_eq!(bitboards[1], 0b0000000_0000000_0000000_0000000_0000000_0000000_0000000);
    }
}