use std::io::stdin;

#[derive(Copy, Clone, PartialEq, Debug)]
enum CellState {
    Player1, Player2, None
}

struct GameField {
    game_field: [CellState; 3*3],
    player: bool,
    slot: u8,
}

impl GameField {
    fn show_field(&self, player1: &char, player2: &char) {
        for row in self.game_field.chunks(3) {
            for c in row {
                match c {
                    CellState::None => {
                        print!(" ");
                    },
                    CellState::Player1 => {
                        print!("{} ", player1);
                    },
                    CellState::Player2 => {
                        print!("{} ", player2);
                    }
                }
            }
            println!();
        }
    }

    fn insert(&mut self, pos: usize) {
        let get_player = self.get_player();
        if let Some(c) = self.game_field.get_mut(pos) {
            if *c == CellState::None {
                *c = get_player;
                self.slot -= 1;
                if !self.check_win() { self.player = !self.player; }
            }
        }
    }

    fn get_player(&self) -> CellState {
        if self.player {
            return CellState::Player1;
        }

        CellState::Player2
    }

    fn check_win(&self) -> bool {
        let get_player = self.get_player();
        (self.game_field[0] == get_player && self.game_field[1] == get_player && self.game_field[2] == get_player) ||
        (self.game_field[3] == get_player && self.game_field[4] == get_player && self.game_field[5] == get_player) ||
        (self.game_field[6] == get_player && self.game_field[7] == get_player && self.game_field[8] == get_player) ||
        (self.game_field[0] == get_player && self.game_field[3] == get_player && self.game_field[6] == get_player) ||
        (self.game_field[1] == get_player && self.game_field[4] == get_player && self.game_field[7] == get_player) ||
        (self.game_field[2] == get_player && self.game_field[5] == get_player && self.game_field[8] == get_player) ||
        (self.game_field[0] == get_player && self.game_field[4] == get_player && self.game_field[8] == get_player) ||
        (self.game_field[2] == get_player && self.game_field[4] == get_player && self.game_field[6] == get_player)
    }
}

fn main() {
    println!("Tic Tac Toe Game");

    let player1 = ask_player_symbol();
    let player2 = ask_player_symbol();

    let mut game_field = GameField {
        game_field: [CellState::None; 3*3],
        player: true,
        slot: 9,
    };

    loop {
        game_field.show_field(&player1, &player2);
        insert_position(&mut game_field);
        if game_field.check_win() {
            game_field.show_field(&player1, &player2);
            break println!("{:?} won!", game_field.get_player());
        } else if game_field.slot == 0 {
            break println!("The winner is... no, there isn't a winner.");
        }
    }
}

fn insert_position(game_field: &mut GameField) {
    let mut pos = String::new();
    println!("Insert the position (0..9):");

    stdin().read_line(&mut pos).expect("Error while reading input.");
    if let Ok(pos) = pos.trim().parse::<usize>() {
        if pos > 0 {
            game_field.insert(pos - 1);
        } else {
            println!("Wrong position!");
        }
    }
}

fn ask_player_symbol() -> char {
    println!("Insert player symbol: ");

    let mut c = String::new();
    stdin().read_line(&mut c).expect("Error while getting input.");

    c.chars().next().unwrap()
}