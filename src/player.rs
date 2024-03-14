mod human_player;
mod random_player;
mod bot_player;
mod bit_bot_player;
mod bit_bot_2_player;
mod human_player_ui;

#[derive(Debug, Clone)]
pub struct Player {
    player: i8,
    player_type: i8
}

impl Player {
    fn new(player: i8, player_type: i8) -> Self {
        println!("player: {} player_type: {}\n", player, player_type);
        Self {
            player,
            player_type,
        }
    }

    pub fn get_player_type(&self) -> i8 {
        self.player_type
    }

    pub fn play(&self, board: &Vec<Vec<i8>>, potential_move: i8) -> i8 {
        match self.player_type {
            5 => return human_player_ui::get_move(&board, potential_move),
            4 => return human_player::get_move(&board),
            3 => return bit_bot_2_player::get_move(&board, self.player),
            2 => return bit_bot_player::get_move(&board, self.player),
            1 => return bot_player::get_move(&board, self.player),
            _ => return random_player::get_move(&board),
        }
    }
}

pub fn _select_player(player: i8) -> Player {
    let mut m: String;
    _print_possible_players();
    loop {
        println!("enter type of player: ");
        m = String::new();
        std::io::stdin().read_line(&mut m).unwrap();
        m = m.trim().to_string();
        match m.parse::<i8>() {
            Ok(_) => break,
            _ => continue,
        }
    }

    let mut m = m.parse::<i8>().unwrap();
    if m < 0 || m > 4 {
        m = 0;
    }
    Player::new(player, m)
}


pub fn _select_player_in_code(player: i8, player_type: i8) -> Player {
    Player::new(player, player_type)
}


fn _print_possible_players() {
    println!("possible players: ");
    println!("0: Random");
    println!("1: Bot");
    println!("2: Bit Bot");
    println!("3: Bit Bot 2");
    println!("4: Human");
}