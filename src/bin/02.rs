use std::{fmt, result};

use ::phf::phf_map;

pub struct GameTurn<'a> {
    opponents_move: &'a str,
    recommended_move: &'a str
}

static MOVE_NAMES: phf::Map<&'static str, &'static str> = phf_map! {
    "A" => "Rock",
    "B" => "Paper",
    "C" => "Scissors",
    "X" => "Rock",
    "Y" => "Paper",
    "Z" => "Scissors"
};

static MOVE_POINTS: phf::Map<&'static str, u32> = phf_map! {
    "Rock" => 1,
    "Paper" => 2,
    "Scissors" => 3
};

// Which other item a selection can vanquish
static WINNING_COMBOS: phf::Map<&'static str, &'static str> = phf_map! {
    "Rock" => "Scissors",
    "Paper" => "Rock",
    "Scissors" => "Paper"
};

// // How to lose! Keys are used to show what will be the selection.
// static LOSING_COMBOS: phf::Map<&'static str, &'static str> = phf_map! {
//     "Scissors" => "Rock",
//     "Rock" => "Paper",
//     "Paper" => "Scissors"
// };


#[derive(Debug, Clone)]
pub struct GameTurnFormatError;

impl fmt::Display for GameTurnFormatError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Game turn information was invalid. Skipping.")
    }
}

pub fn parse_game_turn(turn_data: &str) -> Result<GameTurn, GameTurnFormatError> {
    let split_data: Vec<&str> = turn_data.trim().split(' ').collect();

    match split_data.len() {
        2 => {
            Ok(GameTurn {
                opponents_move: split_data[0],
                recommended_move: split_data[1]
            })
        }
        _ => Err(GameTurnFormatError)
    }
}

pub fn calculate_score(turn_num: usize, game_turn: GameTurn) -> u32 {
    let opponents_move_name = MOVE_NAMES.get(game_turn.opponents_move).cloned().unwrap();
    let my_move_name = MOVE_NAMES.get(game_turn.recommended_move).cloned().unwrap();

    let result_bonus: u32 = {
        // Check for a draw
        if opponents_move_name.eq(my_move_name) {
            return 3;
        }

        // Check to see if I got my butt whooped
        let what_opponent_would_win_over = WINNING_COMBOS.get(opponents_move_name).cloned().unwrap();

        if what_opponent_would_win_over.eq(my_move_name) {
            return 0;
        }

        // I won!
        6
    };


    MOVE_POINTS.get(my_move_name).cloned().unwrap() + result_bonus
}

pub fn part_one(input: &str) -> Option<u32> {
    let total_score: u32 = input.split('\n').enumerate().map(|(i, line)| {
        let game_turn = parse_game_turn(line);

        match game_turn {
            Ok(game_turn) => calculate_score(i, game_turn),
            _ => 0
        }
    })
    .sum();

    Some(total_score)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), None);
    }
}
