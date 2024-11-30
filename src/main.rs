use std::{collections::HashSet, fmt::Display, io};

#[derive(Debug)]
struct Game {
    top_left: i8,
    top_mid: i8,
    top_right: i8,
    mid_left: i8,
    mid_mid: i8,
    mid_right: i8,
    bot_left: i8,
    bot_mid: i8,
    bot_right: i8,
    available_moves: HashSet<u8>,
    active_player: u8,
    move_map: HashSet<u8>,
}

impl Game {
    fn new() -> Self {
        Game {
            top_left: 0,
            top_mid: 0,
            top_right: 0,
            mid_left: 0,
            mid_mid: 0,
            mid_right: 0,
            bot_left: 0,
            bot_mid: 0,
            bot_right: 0,
            available_moves: HashSet::from([1, 2, 3, 4, 5, 6, 7, 8, 9]),
            active_player: 0,
            move_map: HashSet::with_capacity(9),
        }
    }
    fn is_over(&self) -> bool {
        if (self.top_left + self.top_mid + self.top_right).abs() == 3 {
            true
        } else if (self.mid_left + self.mid_mid + self.mid_right).abs() == 3 {
            true
        } else if (self.bot_left + self.bot_mid + self.bot_right).abs() == 3 {
            true
        } else if (self.top_left + self.mid_left + self.bot_left).abs() == 3 {
            true
        } else if (self.top_mid + self.mid_mid + self.bot_mid).abs() == 3 {
            true
        } else if (self.top_right + self.mid_right + self.bot_right).abs() == 3 {
            true
        } else if (self.top_left + self.mid_mid + self.bot_right).abs() == 3 {
            true
        } else if (self.bot_left + self.mid_mid + self.top_right).abs() == 3 {
            true
        } else {
            false
        }
    }
}

fn num_to_str(int_val: i8) -> char {
    if int_val == 1 {
        return 'x';
    } else if int_val == -1 {
        return 'o';
    } else if int_val == 0 {
        return ' ';
    } else {
        unreachable!()
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}|{}|{}\n-----\n{}|{}|{}\n-----\n{}|{}|{}",
            num_to_str(self.top_left),
            num_to_str(self.top_mid),
            num_to_str(self.top_right),
            num_to_str(self.mid_left),
            num_to_str(self.mid_mid),
            num_to_str(self.mid_right),
            num_to_str(self.bot_left),
            num_to_str(self.bot_mid),
            num_to_str(self.bot_right),
        )
    }
}

fn main() {
    let mut game = Game::new();
    while !game.is_over() {
        println!("{}", &game);
        let possible_moves = game.available_moves.difference(&game.move_map);
        let mut possible_moves: Vec<&u8> = possible_moves.into_iter().collect();
        possible_moves.sort();
        println!("Guess a number in {:?}", &possible_moves);
        let input = loop {
            let mut input = String::new();
            let _ = io::stdin().read_line(&mut input);
            let input: u8 = match input.trim().parse() {
                Ok(num) => match game.move_map.get(&num) {
                    None => num,
                    Some(..) => {
                        println!("Not a correct entry, try again!");
                        continue;
                    }
                },
                Err(_) => {
                    println!("Not a correct entry, try again!");
                    continue;
                }
            };
            break input;
        };
        game.move_map.insert(input);
        let x_or_o = match game.active_player {
            0 => 1,
            1 => -1,
            _ => unreachable!(),
        };
        match input {
            1 => game.top_left = x_or_o,
            2 => game.top_mid = x_or_o,
            3 => game.top_right = x_or_o,
            4 => game.mid_left = x_or_o,
            5 => game.mid_mid = x_or_o,
            6 => game.mid_right = x_or_o,
            7 => game.bot_left = x_or_o,
            8 => game.bot_mid = x_or_o,
            9 => game.bot_right = x_or_o,
            _ => unreachable!(),
        };
        game.active_player = match game.active_player {
            0 => 1,
            1 => 0,
            _ => unreachable!(),
        };
    }
    println!("Game is over! Final position: {}", &game);
}
