

pub fn get_move(board: &Vec<Vec<i8>>, player: i8) -> i8 {
    let depth = 10;
    let m;
    if player == 1 {
        m = start_alpha_beta(board, player, depth);
    }else {
        let new_board = flip(board);
        m = start_alpha_beta(&new_board, player*-1, depth);
        return m;
    }
    println!("move: {}", m);
    return m;
}

fn start_alpha_beta(board: &Vec<Vec<i8>>, player: i8, depth: i8) -> i8 {
    let mut best_move = -1;
    let mut alpha = -10000000;
    let beta = 10000000;

    for lm in get_legal_moves(board) {
        let score = alpha_beta(board, player*-1, depth-1, lm, 0, alpha, beta);
        // println!("move: {} score: {} alpha {}", lm, score, alpha);
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


fn alpha_beta(board: &Vec<Vec<i8>>, player: i8, depth: i8, m: i8, old_score: i32, mut alpha: i32, mut beta: i32) -> i32 {
    let mut new_board = board.clone();
    let mut score = calc_score(&mut new_board, player*-1, m);

    if score == 1000 {
        if player == -1 {
            return score;
        } else {
            return (score - (100 - depth as i32)) * -1;
        }
    }
    score = (score * player as i32 * -1) + old_score;
    if depth == 0 {
        return score;
    }
    
    //println!("board: {:?}", new_board);
    for lm in get_legal_moves(&new_board) {
        let new_score = alpha_beta(&new_board, player*-1, depth-1, lm, score, alpha, beta);
        if player == 1 {
            if new_score > alpha {
                alpha = new_score;
                if alpha >= beta || alpha == 1000 {
                    return alpha;
                }
            }
        } else {
            if new_score < beta {
                beta = new_score;
                if alpha >= beta {
                    return beta;
                }
            }
        }
    }

    if player == 1 {
        return alpha;
    } else {
        return beta;
    }
}


fn calc_score(board: &mut Vec<Vec<i8>>, player: i8, m: i8) -> i32 {
    let row = update_board(board, player, m);
    let mut con = vec![1,1,1,1];

    //col
    for i in row+1..6.min(row+4) {
        if board[i][m as usize] == player {
            con[0] += 1;
        } else {
            break;
        }
    }
    //row
    for i in m+1..7.min(m+4) {
        if board[row][i as usize] == player {
            con[1] += 1;
        } else {
            break;
        }
    }
    for i in (0.max(m-3)..m).rev() {
        if board[row][i as usize] == player {
            con[1] += 1;
        } else {
            break;
        }
    }
    //sid_1
    for i in 1..4 {
        if row+i > 5 || m as usize +i > 6 {
            break;
        }
        if board[row+i][m as usize +i] == player {
            con[2] += 1;
        } else {
            break;
        }
    }
    for i in 1..4 {
        if (row as i8)-(i as i8) < 0 || m-(i as i8) < 0 {
            break;
        }
        if board[row-i][m as usize -i] == player {
            con[2] += 1;
        } else {
            break;
        }
    }
    //sid_2
    for i in 1..4 {
        if row+i > 5 || m-(i as i8) < 0 {
            break;
        }
        if board[row+i][m as usize -i] == player {
            con[3] += 1;
        } else {
            break;
        }
    }
    for i in 1..4 {
        if (row as i8)-(i as i8) < 0 || m as usize +i > 6 {
            break;
        }
        if board[row-i][m as usize +i] == player {
            con[3] += 1;
        } else {
            break;
        }
    }

    //println!("con: {:?} move: {} player: {}", con, m, player);
    if 4 <= *con.iter().max().unwrap() {
        //println!("win move: {}", m);
        return 1000;
    }
    return con.iter().sum();
}


fn update_board(board: &mut Vec<Vec<i8>>, player: i8, m: i8) -> usize {
    for i in (0..6).rev() {
        if board[i][m as usize] == 0 {
            board[i][m as usize] = player;
            return i;
        }
    }
    panic!("movee got an invalid move '{}'", m);
}


fn get_legal_moves(board: &Vec<Vec<i8>>) -> Vec<i8> {
    let mut legal_moves = Vec::new();
    for i in 0..7 {
        if board[0][i as usize] == 0 {
            legal_moves.push(i);
        }
    }
    legal_moves
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


    #[test]
    fn test_new_board_test() {
        let board = new_board();
        assert_eq!(board.len(), 6);
        assert_eq!(board[0].len(), 7);
    }

    #[test]
    fn test_get_legal_moves() {
        let mut board = new_board();
        let mut legal_moves = get_legal_moves(&board);
        assert_eq!(legal_moves.len(), 7);
        board[0][3] = 1;
        legal_moves = get_legal_moves(&board);
        assert_eq!(legal_moves.len(), 6);
    }

    #[test]
    fn test_find_win_1() {
        let mut board = new_board();
        board[5] = vec![1, 0, 0, 0, 0, 0, 0];
        board[4] = vec![1, 0, 0, 0, 0, 0, 0];
        board[3] = vec![1, 0, 0, 0, 0, 0, 0];
        assert_eq!(calc_score(&mut board, 1, 0), 1000);
    }

    #[test]
    fn test_find_win_2() {
        let mut board = new_board();
        board[5] = vec![1, 1, 1, 0, 0, 0, 0];
        assert_eq!(calc_score(&mut board, 1, 3), 1000);
    }

    #[test]
    fn test_find_win_3() {
        let mut board = new_board();
        board[2] = vec![0, 0, 0, 0, 0, 0, 0];
        board[3] = vec![0, 0, 1, -1, 0, 0, 0];
        board[4] = vec![0, 1, -1, 1, 0, 0, 0];
        board[5] = vec![1, -1, 1, -1, 0, 0, 0];
        assert_eq!(calc_score(&mut board, 1, 3), 1000);
    }

    #[test]
    fn test_find_win_4() {
        let mut board = new_board();
        board[2] = vec![0, 0, 0, 0, 0, 0, 0];
        board[3] = vec![-1, 1, 0, 0, 0, 0, 0];
        board[4] = vec![1, -1, 1, 0, 0, 0, 0];
        board[5] = vec![-1, 1, -1, 1, 0, 0, 0];
        assert_eq!(calc_score(&mut board, 1, 0), 1000);
    }

    #[test]
    fn test_alpha_beta_1() {
        let mut board = new_board();
        board[2] = vec![0, 0, 0, 0, 0, 0, 0];
        board[3] = vec![0, 0, 0, -1, 0, 0, 0];
        board[4] = vec![0, 1, 1, 1, 0, 0, 0];
        board[5] = vec![1, -1, 1, -1, 0, 0, 0];
        assert_eq!(alpha_beta(&mut board, 1*-1, 5, 2, 0, -100000, 100000), 1000);
    }

    #[test]
    fn test_alpha_beta_2() {
        let mut board = new_board();
        board[2] = vec![0, 0, 0, 0, 0, 0, 0];
        board[3] = vec![0, 0, 1, -1, 0, 0, 0];
        board[4] = vec![0, 1, 1, 1, 0, 0, 0];
        board[5] = vec![1, -1, 1, -1, 0, 0, 0];
        assert_eq!(alpha_beta(&mut board, -1*-1, 5, 6, 0, -100000, 100000), 1000);
    }

    #[test]
    fn test_find_lose() {
        let mut board = new_board();
        board[4] = vec![0, 0, 0, 0, 0, 0, 0];
        board[5] = vec![0, 1, 1, -1, -1, 0, -1];
        assert_eq!(alpha_beta(&mut board, 1*-1, 1, 2, 0, -100000, 100000), -900);
    }

}