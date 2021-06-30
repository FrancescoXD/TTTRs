use std::{io::stdin};

fn main() {
    println!("Tic Tac Toe Game");

    let player1 = ask_player_char();
    let player2 = ask_player_char();

    let mut turn = 0;
    let mut table = [' '; 9];
    let mut empty_slot: u8 = 9;
    
    loop {
        show_board(&table[..]);
        let symbol = select_player(&mut turn, player1, player2);
        insert_symbol(&mut empty_slot, symbol, &mut table[..], &mut turn);

        if check_win(symbol, empty_slot, &mut table[..]) {
            show_board(&table[..]);
            if empty_slot == 0 {
                break println!("Nobody is the winner!");
            }
            break println!("{} won!", symbol);
        }
    }
}

fn insert_symbol(empty_slot: &mut u8, symbol: char, table: &mut [char], turn: &mut i32) {
    let mut pos = String::new();

    println!("Insert the position ({}):", symbol);
    stdin().read_line(&mut pos).expect("Error while getting input.");
    let mut pos: usize = pos.trim().parse().expect("Error while parsing the number.");

    if pos != 0 {
        pos -= 1;
    }

    if is_valid_position(pos, &table[..]) {
        table[pos] = symbol;
        *turn += 1;
        *empty_slot -= 1;
    }
}

fn ask_player_char() -> char {
    println!("Insert player symbol: ");

    let mut c = String::new();
    stdin().read_line(&mut c).expect("Error while getting input.");

    let symbol: char = c.chars().next().unwrap();
    symbol
}

fn select_player(turn: &mut i32, player1: char, player2: char) ->  char {
    if *turn % 2 == 0 {
        return player1;
    }

    player2
}

fn show_board(table: &[char]) {
    for (i, &item) in table.iter().enumerate() {
        print!("{} ", item);
        if i == 2 || i == 5 || i == 8 {
            println!();
        }
    }
}

fn is_valid_position(pos: usize, table: &[char]) -> bool {
    if table[pos] == ' ' {
        return true;
    }

    false
}

fn check_win(symbol: char, empty_slot: u8, table: &mut [char]) -> bool {
    if (empty_slot == 0) ||
    (table[0] == symbol && table[1] == symbol && table[2] == symbol) ||
    (table[3] == symbol && table[4] == symbol && table[5] == symbol) ||
    (table[6] == symbol && table[7] == symbol && table[8] == symbol) ||
    (table[0] == symbol && table[3] == symbol && table[6] == symbol) ||
    (table[1] == symbol && table[4] == symbol && table[7] == symbol) ||
    (table[2] == symbol && table[5] == symbol && table[8] == symbol) ||
    (table[0] == symbol && table[4] == symbol && table[8] == symbol) ||
    (table[2] == symbol && table[4] == symbol && table[6] == symbol) {
        return true;
    }

    false
}