use std::{fmt::Display, io};

const MOVES: [&'static str; 9] = ["1", "2", "3", "4", "5", "6", "7", "8", "9"];

#[derive(Debug)]
enum Player {
    X,
    O,
}

#[derive(Debug, Clone, Copy)]
enum Tile {
    X,
    O,
    EMPTY,
}

#[derive(Debug)]
struct Game {
    board: [Tile; 9],
    moves: [usize; 9],
    active_player: Player,
}

impl Game {
    fn new() -> Self {
        Game {
            board: [Tile::EMPTY; 9],
            moves: [0, 1, 2, 3, 4, 5, 6, 7, 8],
            active_player: Player::X,
        }
    }
    fn get_possible_moves(&self) -> Vec<&str> {
        self.board
            .iter()
            .zip(self.moves)
            .filter(|(value, _)| matches!(**value, Tile::EMPTY))
            .map(|(_, c)| MOVES[c])
            .collect()
    }
    fn is_over(&self) -> bool {
        self.is_win_x() | self.is_win_o() | self.is_draw()
    }
    fn is_win_x(&self) -> bool {
        match self.board {
            [Tile::X, Tile::X, Tile::X, _, _, _, _, _, _] => true,
            [_, _, _, _, _, _, Tile::X, Tile::X, Tile::X] => true,
            [_, _, _, Tile::X, Tile::X, Tile::X, _, _, _] => true,
            [Tile::X, _, _, Tile::X, _, _, Tile::X, _, _] => true,
            [_, Tile::X, _, _, Tile::X, _, _, Tile::X, _] => true,
            [_, _, Tile::X, _, _, Tile::X, _, _, Tile::X] => true,
            [Tile::X, _, _, _, Tile::X, _, _, _, Tile::X] => true,
            [_, _, Tile::X, _, Tile::X, _, Tile::X, _, _] => true,
            _ => false,
        }
    }
    fn is_win_o(&self) -> bool {
        match self.board {
            [Tile::O, Tile::O, Tile::O, _, _, _, _, _, _] => true,
            [_, _, _, _, _, _, Tile::O, Tile::O, Tile::O] => true,
            [_, _, _, Tile::O, Tile::O, Tile::O, _, _, _] => true,
            [Tile::O, _, _, Tile::O, _, _, Tile::O, _, _] => true,
            [_, Tile::O, _, _, Tile::O, _, _, Tile::O, _] => true,
            [_, _, Tile::O, _, _, Tile::O, _, _, Tile::O] => true,
            [Tile::O, _, _, _, Tile::O, _, _, _, Tile::O] => true,
            [_, _, Tile::O, _, Tile::O, _, Tile::O, _, _] => true,
            _ => false,
        }
    }
    fn is_draw(&self) -> bool {
        if self.get_possible_moves().is_empty() {
            true
        } else {
            false
        }
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display_vals: Vec<&str> = self
            .board
            .iter()
            .zip(self.moves)
            .map(|(value, index)| match *value {
                Tile::EMPTY => MOVES[index],
                Tile::X => "x",
                Tile::O => "o",
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
            Player::X => Tile::X,
            Player::O => Tile::O,
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
        if game.is_draw() {
            println!("Game is a draw!");
            break;
        } else if game.is_win_x() {
            println!("Player {:?} won!", Player::X);
            println!("Final position: {}", &game);
            break;
        } else if game.is_win_o() {
            println!("Player {:?} won!", Player::O);
            println!("Final position: {}", &game);
            break;
        } else {
            game.active_player = match game.active_player {
                Player::X => Player::O,
                Player::O => Player::X,
            }
        }
    }
    //println!("Game is over! Final position: {}", &game);
}
