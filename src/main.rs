use std::{fmt::Display, io};

#[derive(Debug)]
struct Game {
    board: [u8; 9],
    moves: [&'static str; 9],
    active_player: u8,
}

impl Game {
    fn new() -> Self {
        Game {
            board: [0; 9],
            moves: ["1", "2", "3", "4", "5", "6", "7", "8", "9"],
            active_player: 0,
        }
    }
    fn get_possible_moves(&self) -> Vec<&str> {
        self.board
            .iter()
            .zip(self.moves)
            .filter(|(value, _)| **value == 0)
            .map(|(_, c)| c)
            .collect()
    }
}

fn num_to_str(int_val: u8) -> &'static str {
    match int_val {
        10 => "x",
        20 => "o",
        _ => unreachable!(),
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display_vals: Vec<&str> = self
            .board
            .iter()
            .zip(self.moves)
            .map(|(value, index)| {
                if *value == 0 {
                    index
                } else {
                    num_to_str(*value)
                }
            })
            .collect();
        write!(
            f,
            "\n{}|{}|{}\n-----\n{}|{}|{}\n-----\n{}|{}|{}\n",
            display_vals[0],
            display_vals[1],
            display_vals[2],
            display_vals[3],
            display_vals[4],
            display_vals[5],
            display_vals[6],
            display_vals[7],
            display_vals[8],
        )
    }
}

fn main() {
    let mut game = Game::new();
    loop {
        println!("{}", &game);
        let possible_moves = game.get_possible_moves();
        println!("Possible moves: {:?}", &possible_moves);
        let input = loop {
            let mut input = String::new();
            let _ = io::stdin().read_line(&mut input);
            let input = if possible_moves.contains(&input.trim()) {
                input
            } else {
                println!("Not a correct entry, try again!");
                continue;
            };
            break input;
        };
        let x_or_o = match game.active_player {
            0 => 10,
            1 => 20,
            _ => unreachable!(),
        };
        match input.trim() {
            "1" => game.board[0] = x_or_o,
            "2" => game.board[1] = x_or_o,
            "3" => game.board[2] = x_or_o,
            "4" => game.board[3] = x_or_o,
            "5" => game.board[4] = x_or_o,
            "6" => game.board[5] = x_or_o,
            "7" => game.board[6] = x_or_o,
            "8" => game.board[7] = x_or_o,
            "9" => game.board[8] = x_or_o,
            _ => unreachable!(),
        };
        game.active_player = match game.active_player {
            0 => 1,
            1 => 0,
            _ => unreachable!(),
        };
    }
    //println!("Game is over! Final position: {}", &game);
}
