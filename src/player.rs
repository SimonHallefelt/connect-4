mod human_player;
mod random_player;
mod bot_player;

pub struct Player {
    player: i8,
    player_type: i8
}

impl Player {
    fn new(player: i8, player_type: i8) -> Self {
        println!("player: {} player_type: {}", player, player_type);
        Self {
            player,
            player_type,
        }
    }

    pub fn play(&self, board: &Vec<Vec<i8>>) -> i8 {
        match self.player_type {
            2 => return bot_player::get_move(&board, self.player),
            1 => return human_player::get_move(&board),
            _ => return random_player::get_move(&board),
        }
    }
}

pub fn select_player(player: i8) -> Player {
    let mut m: String = String::new();
    print_possible_players();
    println!("enter your type of player: ");
    loop {
        m = String::new();
        print!("enter your move: ");
        std::io::stdin().read_line(&mut m).unwrap();
        m = m.trim().to_string();
        println!();
        match m.parse::<i8>() {
            Ok(_) => break,
            _ => continue,
        }
    }

    Player::new(player, m.parse::<i8>().unwrap())
}


fn print_possible_players() {
    println!("possible players: ");
    println!("0: Random");
    println!("1: Human");
    println!("2: Bot");
}