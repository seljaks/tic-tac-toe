use std::{cmp, fmt::Display, io, usize};

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
}

impl Game {
    fn new() -> Self {
        Game {
            board: [Tile::EMPTY; 9],
            active_player: Player::X,
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
    fn get_state(&self) -> GameState {
        if self.is_win_x() {
            GameState::WinX
        } else if self.is_win_o() {
            GameState::WinO
        } else if self.is_draw() {
            GameState::Draw
        } else {
            GameState::Ongoing
        }
    }
    fn play_turn(&mut self) -> () {
        match self.get_state() {
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
                let (position, tile) = self.get_user_input();
                self.play_tile(position, tile);
                self.switch_player();
            }
        }
    }
    fn switch_player(&mut self) -> () {
        self.active_player = match self.active_player {
            Player::X => Player::O,
            Player::O => Player::X,
        };
    }
    fn get_user_input(&self) -> (usize, Tile) {
        println!("{}", self);
        let possible_moves = &self.get_available_moves();
        println!("Possible moves: {:?}", possible_moves);
        let position = loop {
            let mut input = String::new();
            let _ = io::stdin().read_line(&mut input);
            let input: usize = if possible_moves.contains(&input.trim()) {
                input.trim().parse().unwrap()
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
        (position, x_or_o)
    }

    fn play_tile(&mut self, position: usize, tile: Tile) -> () {
        match position {
            1..=9 => self.board[position - 1] = tile,
            _ => unreachable!(),
        };
    }
    fn is_over(&self) -> bool {
        matches!(
            self.get_state(),
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

fn generate_game_tree() -> Vec<Vec<Game>> {
    let new_game = Game::new();
    vec![vec![new_game]]
}

fn find_best_move(game: &mut Game, player: Player) -> isize {
    game.get_state();
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
        game.play_turn();
    }
    game.play_turn();
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
