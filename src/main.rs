use std::{cmp, fmt::Display, io};

const MOVES: [&'static str; 9] = ["1", "2", "3", "4", "5", "6", "7", "8", "9"];

#[derive(Debug, Clone, PartialEq, Copy)]
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

#[derive(Debug, Clone)]
enum GameState {
    Ongoing,
    Draw,
    WinX,
    WinO,
}

#[derive(Debug, Clone)]
struct Game {
    board: [Tile; 9],
    active_player: Player,
    state: GameState,
}

impl Game {
    fn new() -> Self {
        Game {
            board: [Tile::EMPTY; 9],
            active_player: Player::O,
            state: GameState::Ongoing,
        }
    }
    fn get_available_moves(&self) -> Vec<&str> {
        self.board
            .iter()
            .enumerate()
            .filter(|(_, val)| matches!(val, Tile::EMPTY))
            .map(|(i, _)| MOVES[i])
            .collect()
    }
    fn _get_moves(&self) -> Vec<usize> {
        self.board
            .iter()
            .enumerate()
            .filter(|(_, val)| matches!(val, Tile::EMPTY))
            .map(|(i, _)| i)
            .collect()
    }
    fn update_state(&mut self) -> () {
        self.state = if self.is_win_x() {
            GameState::WinX
        } else if self.is_win_o() {
            GameState::WinO
        } else if self.is_draw() {
            GameState::Draw
        } else {
            GameState::Ongoing
        }
    }
    fn check_state(&mut self) -> () {
        match self.state {
            GameState::WinX => {
                println!("Player using x won!");
                println!("Final board: {}", &self);
            }
            GameState::WinO => {
                println!("Player using o won!");
                println!("Final board: {}", &self)
            }
            GameState::Draw => {
                println!("Game is drawn.");
                println!("Final board: {}", &self)
            }
            GameState::Ongoing => {
                self.active_player = match self.active_player {
                    Player::X => Player::O,
                    Player::O => Player::X,
                };
                self.get_user_input();
            }
        }
    }
    fn get_user_input(&mut self) -> () {
        println!("{}", self);
        let possible_moves = &self.get_available_moves();
        println!("Possible moves: {:?}", possible_moves);
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
        let x_or_o = match self.active_player {
            Player::X => Tile::X,
            Player::O => Tile::O,
        };
        match input.trim() {
            "1" => self.board[0] = x_or_o,
            "2" => self.board[1] = x_or_o,
            "3" => self.board[2] = x_or_o,
            "4" => self.board[3] = x_or_o,
            "5" => self.board[4] = x_or_o,
            "6" => self.board[5] = x_or_o,
            "7" => self.board[6] = x_or_o,
            "8" => self.board[7] = x_or_o,
            "9" => self.board[8] = x_or_o,
            _ => unreachable!(),
        };
    }
    fn is_over(&self) -> bool {
        matches!(
            self.state,
            GameState::Draw | GameState::WinX | GameState::WinO
        )
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
        if self.get_available_moves().is_empty() {
            true
        } else {
            false
        }
    }
}

fn find_best_move(game: &mut Game, player: Player) -> isize {
    game.update_state();
    dbg!(&game);
    if game.is_over() {
        match game.active_player {
            Player::X => {
                if game.is_win_x() {
                    100
                } else if game.is_win_o() {
                    -100
                } else if game.is_draw() {
                    0
                } else {
                    unreachable!()
                }
            }
            Player::O => {
                if game.is_win_o() {
                    100
                } else if game.is_win_x() {
                    -100
                } else if game.is_draw() {
                    0
                } else {
                    unreachable!()
                }
            }
        }
    } else {
        if game.active_player == player {
            let mut value = -1000;
            for move_idx in game._get_moves().iter() {
                let x_or_o = match player {
                    Player::X => Tile::X,
                    Player::O => Tile::O,
                };
                let mut current_game = game.clone();
                current_game.board[*move_idx] = x_or_o;
                let player = match game.active_player {
                    Player::X => Player::O,
                    Player::O => Player::X,
                };
                current_game.active_player = player;
                value = cmp::max(value, find_best_move(&mut current_game, player));
            }
            value
        } else if game.active_player != player {
            let mut value = 1000;
            for move_idx in game._get_moves().iter() {
                let x_or_o = match player {
                    Player::X => Tile::X,
                    Player::O => Tile::O,
                };
                let mut current_game = game.clone();
                current_game.board[*move_idx] = x_or_o;
                let player = match game.active_player {
                    Player::X => Player::O,
                    Player::O => Player::X,
                };
                current_game.active_player = player;
                value = cmp::min(value, find_best_move(&mut current_game, player));
            }
            value
        } else {
            unreachable!()
        }
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display_vals: Vec<&str> = self
            .board
            .iter()
            .enumerate()
            .map(|(index, value)| match *value {
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
    while !game.is_over() {
        game.check_state();
        game.update_state();
    }
    game.check_state();
}

#[cfg(test)]
mod tests {
    use super::*;

    // o x x
    // x x o
    // o o x
    #[test]
    fn best_move_one_before_win() {
        let mut game = Game::new();
        game.board[0] = Tile::O;
        game.board[1] = Tile::X;
        game.board[2] = Tile::X;
        game.board[3] = Tile::X;
        game.board[4] = Tile::X;
        game.board[5] = Tile::O;
        game.board[6] = Tile::O;
        game.active_player = Player::X;
        let player = game.active_player.clone();
        let val = find_best_move(&mut game, player);
        assert_eq!(val, 100);
    }
}
