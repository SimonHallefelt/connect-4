use rand::seq::SliceRandom;


pub fn get_move(board: &Vec<Vec<i8>>, player: i8) -> i8 {
    let legal_moves = get_legal_moves(board);
    let m = legal_moves.choose(&mut rand::thread_rng());
    return *m.unwrap();
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
}