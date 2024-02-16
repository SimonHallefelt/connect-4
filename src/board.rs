

pub fn new_board() -> Vec<Vec<i8>> {
    let mut board = Vec::new();
    for i in 0..6 {
        board.push(Vec::new());
        for _ in 0..7 {
            board[i].push(0);
        }
    }
    board
}


/*
 * -2: player 1 illegal move
 * -1: player -1 win
 *  0: continue
 *  1: player 1 win
 *  2: player -1 illegal move
 *  3: draw
 */
pub fn update_board(board: &mut Vec<Vec<i8>>, m: i8, player: i8) -> i8 {
    if !legal_move(board, m){
        println!("\nNot a legal move '{}'", m);
        println!("The winning player is '{}'", player * -1);
        print_board(board);
        return player * -1 * 2;
    }
    let _r = movee(board, m, player);
    let win = find_win(board);
    if win {
        println!("\nThe winning player is '{}'", player);
        print_board(board);
        return player;
    }
    let draw  = find_draw(board);
    if draw {
        println!("\nThe game ended in a draw");
        print_board(board);
        return 3;
    }
    0
}


fn legal_move(board: &Vec<Vec<i8>>, m: i8) -> bool {
    for i in 0..7 {
        if board[0][i as usize] == 0 && i == m {
            return true;
        }
    }
    false
}


fn movee(board: &mut Vec<Vec<i8>>, m: i8, player: i8) -> usize {
    for i in (0..6).rev() {
        if board[i][m as usize] == 0 {
            board[i][m as usize] = player;
            return i;
        }
    }
    panic!("movee got an invalid move '{}'", m);
}


fn find_win(board: &Vec<Vec<i8>>) -> bool {
    for i in 3..6 {
        for j in 0..7 {
            if board[i][j] != 0 {
                if board[i][j] == board[i-1][j] && board[i][j] == board[i-2][j] && board[i][j] == board[i-3][j] {
                    return true;
                }
            }
        }
    }
    for i in 0..6 {
        for j in 0..4 {
            if board[i][j] != 0 {
                if board[i][j] == board[i][j+1] && board[i][j] == board[i][j+2] && board[i][j] == board[i][j+3] {
                    return true;
                }
            }
        }
    }
    for i in 3..6 {
        for j in 0..4 {
            if board[i][j] != 0 {
                if board[i][j] == board[i-1][j+1] && board[i][j] == board[i-2][j+2] && board[i][j] == board[i-3][j+3] {
                    return true;
                }
            }
        }
    }
    for i in 0..3 {
        for j in 0..4 {
            if board[i][j] != 0 {
                if board[i][j] == board[i+1][j+1] && board[i][j] == board[i+2][j+2] && board[i][j] == board[i+3][j+3] {
                    return true;
                }
            }
        }
    }
    false
}


fn find_draw(board: &Vec<Vec<i8>>) -> bool {
    for i in 0..7 {
        if board[0][i] == 0{
            return false;
        }
    }
    true
}


pub fn print_board(board: &Vec<Vec<i8>>) {
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

    #[test]
    fn test_new_board() {
        let board = new_board();
        assert_eq!(board.len(), 6);
        assert_eq!(board[0].len(), 7);
    }


    #[test]
    fn test_movee() {
        let mut board = new_board();
        let r = movee(&mut board, 0, 1);
        assert_eq!(board[5][0], 1);
        assert_eq!(board[5][1], 0);
        assert_eq!(board[4][0], 0);
        assert_eq!(r, 5);
        let r = movee(&mut board, 0, 1);
        assert_eq!(board[5][0], 1);
        assert_eq!(board[4][0], 1);
        assert_eq!(board[3][0], 0);
        assert_eq!(r, 4);
    }


    #[test]
    fn test_legal_move() {
        let mut board = new_board();
        assert!(legal_move(&mut board, 0));
        assert!(!legal_move(&mut board, -1));
        assert!(!legal_move(&mut board, 7));
        movee(&mut board, 0, 1);
        movee(&mut board, 0, 1);
        movee(&mut board, 0, 1);
        movee(&mut board, 0, 1);
        movee(&mut board, 0, 1);
        assert!(legal_move(&mut board, 0));
        movee(&mut board, 0, 1);
        assert!(!legal_move(&mut board, 0));
    }

    #[test]
    fn test_find_win_1() {
        let mut board = new_board();
        movee(&mut board, 0, 1);
        movee(&mut board, 0, 1);
        movee(&mut board, 0, 1);
        movee(&mut board, 0, 1);
        assert!(find_win(&mut board));
    }

    #[test]
    fn test_find_win_2() {
        let mut board = new_board();
        movee(&mut board, 0, 1);
        movee(&mut board, 1, 1);
        movee(&mut board, 2, 1);
        movee(&mut board, 3, 1);
        assert!(find_win(&mut board));
    }

    #[test]
    fn test_find_win_3() {
        let mut board = new_board();
        board[2] = vec![0, 0, 0, 1, 0, 0, 0];
        board[3] = vec![0, 0, 1, -1, 0, 0, 0];
        board[4] = vec![0, 1, -1, 1, 0, 0, 0];
        board[5] = vec![1, -1, 1, -1, 0, 0, 0];
        assert!(find_win(&mut board));
    }

    #[test]
    fn test_find_win_4() {
        let mut board = new_board();
        board[2] = vec![1, 0, 0, 0, 0, 0, 0];
        board[3] = vec![-1, 1, 0, 0, 0, 0, 0];
        board[4] = vec![1, -1, 1, 0, 0, 0, 0];
        board[5] = vec![-1, 1, -1, 1, 0, 0, 0];
        assert!(find_win(&board));
    }

    #[test]
    fn test_draw() {
        let mut board = new_board();
        assert!(!find_draw(&board));
        board[0] = vec![ 1,  1, -1, -1,  1,  1, -1];
        board[1] = vec![-1, -1,  1,  1, -1, -1,  1];
        board[2] = vec![ 1,  1, -1, -1,  1,  1, -1];
        board[3] = vec![-1, -1,  1,  1, -1, -1,  1];
        board[2] = vec![ 1,  1, -1, -1,  1,  1, -1];
        board[5] = vec![-1, -1,  1,  1, -1, -1,  1];
        assert!(find_draw(&board));
    }

}