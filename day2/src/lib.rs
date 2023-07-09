use std::str::FromStr;

#[derive(Debug)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for Move {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Self::Rock),
            "B" => Ok(Self::Paper),
            "C" => Ok(Self::Scissors),
            "X" => Ok(Self::Rock),
            "Y" => Ok(Self::Paper),
            "Z" => Ok(Self::Scissors),
            _ => Ok(Self::Rock),
        }
    }
}

impl Move {
    fn get_score_for_move(self) -> usize {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
}

#[derive(Debug)]
enum Outcome {
    Win,
    Loss,
    Draw,
}

impl FromStr for Outcome {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Self::Loss),
            "Y" => Ok(Self::Draw),
            "Z" => Ok(Self::Win),
            _ => Ok(Self::Loss),
        }
    }
}

impl Outcome {
    // Computes the points based on the tuple of the round
    fn get_outcome_for_round(round: &(Move, Move)) -> Self {
        match round {
            (Move::Rock, Move::Rock) => Self::Draw,
            (Move::Rock, Move::Paper) => Self::Win,
            (Move::Rock, Move::Scissors) => Self::Loss,
            (Move::Paper, Move::Rock) => Self::Loss,
            (Move::Paper, Move::Paper) => Self::Draw,
            (Move::Paper, Move::Scissors) => Self::Win,
            (Move::Scissors, Move::Rock) => Self::Win,
            (Move::Scissors, Move::Paper) => Self::Loss,
            (Move::Scissors, Move::Scissors) => Self::Draw,
        }
    }

    // Returns the move to play for the provided round
    fn get_move_for_round(round: &(Move, Outcome)) -> Move {
        match round {
            (Move::Rock, Outcome::Win) => Move::Paper,
            (Move::Rock, Outcome::Draw) => Move::Rock,
            (Move::Rock, Outcome::Loss) => Move::Scissors,
            (Move::Paper, Outcome::Win) => Move::Scissors,
            (Move::Paper, Outcome::Draw) => Move::Paper,
            (Move::Paper, Outcome::Loss) => Move::Rock,
            (Move::Scissors, Outcome::Win) => Move::Rock,
            (Move::Scissors, Outcome::Draw) => Move::Scissors,
            (Move::Scissors, Outcome::Loss) => Move::Paper,
        }
    }

    // Returns the score for a given outcome
    fn get_score_for_outcome(self) -> usize {
        match self {
            Self::Loss => 0,
            Self::Draw => 3,
            Self::Win => 6,
        }
    }
}

// Computes the answer for part 1
pub fn part_1(input: String) -> usize {
    input
        .split("\n")
        .map(|e| {
            let moves = e.split(" ").collect::<Vec<&str>>();
            let round = (
                Move::from_str(moves[0]).unwrap_or(Move::Rock),
                Move::from_str(moves[1]).unwrap_or(Move::Rock),
            );

            Outcome::get_outcome_for_round(&round).get_score_for_outcome()
                + round.1.get_score_for_move()
        })
        .sum::<usize>()
}

pub fn part_2(input: String) -> usize {
    input
        .split("\n")
        .map(|r| {
            let r_moves = r.split(" ").collect::<Vec<&str>>();
            let round = (
                Move::from_str(r_moves[0]).unwrap_or(Move::Rock),
                Outcome::from_str(r_moves[1]).unwrap_or(Outcome::Loss),
            );

            Outcome::get_move_for_round(&round).get_score_for_move()
                + round.1.get_score_for_outcome()
        })
        .sum::<usize>()
}

#[cfg(test)]
mod day_2_test {
    use super::*;
    use utils::parse_input;

    #[test]
    fn part_1_example() {
        let content = parse_input("example.txt").unwrap();
        let result = part_1(content);
        assert_eq!(result, 15);
    }

    #[test]
    fn part_1_input() {
        let content = parse_input("input.txt").unwrap();
        let result = part_1(content);
        assert_eq!(result, 15632);
    }

    #[test]
    fn part_2_example() {
        let content = parse_input("example.txt").unwrap();
        let result = part_2(content);
        assert_eq!(result, 12);
    }

    #[test]
    fn part_2_input() {
        let content = parse_input("input.txt").unwrap();
        let result = part_2(content);
        assert_eq!(result, 14416);
    }
}
