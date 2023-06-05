use std::{fmt};

use ::phf::phf_map;

pub struct GameTurn<'a> {
    opponents_move: &'a str,
    desired_outcome: &'a str
}

static MOVE_NAMES: phf::Map<&'static str, &'static str> = phf_map! {
    "A" => "Rock",
    "B" => "Paper",
    "C" => "Scissors"
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

// What items are needed to pick to lose to the keys
static LOSING_COMBOS: phf::Map<&'static str, &'static str> = phf_map! {
    "Scissors" => "Rock",
    "Rock" => "Paper",
    "Paper" => "Scissors"
};


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
                desired_outcome: split_data[1]
            })
        }
        _ => Err(GameTurnFormatError)
    }
}

pub fn locate_move_points(move_name: &str) -> u32 {
    MOVE_POINTS.get(move_name).cloned().unwrap()
}

pub fn determine_my_moves_points(game_turn: GameTurn) -> u32 {
    let opponents_move_name = MOVE_NAMES.get(game_turn.opponents_move).cloned().unwrap();

    match game_turn.desired_outcome {
        "X" =>  {
            // Get the move name that would allow my opponent to win
            locate_move_points(WINNING_COMBOS.get(opponents_move_name).cloned().unwrap())
        },
        "Y" => {
            //Draw. Ez pz.
            locate_move_points(opponents_move_name)
        },
        "Z" => {
            // Get the move name that would have me be the victor.
            locate_move_points(LOSING_COMBOS.get(opponents_move_name).cloned().unwrap())
        },
        // I should probably throw a fit here if some other value is passed.
        _ => 0
    }
}

pub fn calculate_score(game_turn: GameTurn) -> u32 {
    let result_bonus: u32 = match game_turn.desired_outcome {
        "X" => 0,
        "Y" => 3,
        "Z" => 6,
        _ => 0
    };

    determine_my_moves_points(game_turn) + result_bonus
}

pub fn part_one(input: &str) -> Option<u32> {
    let total_score: u32 = input.split('\n').map(|line| {
        let game_turn = parse_game_turn(line);

        match game_turn {
            Ok(game_turn) => calculate_score(game_turn),
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
