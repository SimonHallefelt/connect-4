mod random_player;
mod bot_player;

pub struct Player {
    player: i8,
    player_type: i8
}

impl Player {
    pub fn new(player: i8, player_type: i8) -> Self {
        Self {
            player,
            player_type,
        }
    }

    pub fn play(&self, board: &Vec<Vec<i8>>) -> i8 {
        match self.player_type {
            1 => return bot_player::get_move(&board, self.player),
            _ => return random_player::get_move(&board),
        }
    }
}