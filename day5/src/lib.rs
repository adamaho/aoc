use std::{collections::VecDeque, str::FromStr};

#[derive(Debug, Default)]
struct Move {
    amount: usize,
    start: usize,
    end: usize,
}

#[derive(Clone, PartialEq)]
pub enum MoveMode {
    OneByOne,
    All,
}

impl Move {
    pub fn apply_move(
        self: Self,
        stacks: &mut Vec<VecDeque<String>>,
        mode: MoveMode,
    ) -> &mut Vec<VecDeque<String>> {
        let start_vec: Vec<String> = stacks[self.start].clone().into();

        let mut crates_to_move = vec![String::new(); self.amount];
        crates_to_move.clone_from_slice(&start_vec[0..self.amount]);

        if mode == MoveMode::All {
            crates_to_move.reverse();
        }

        for c in &crates_to_move {
            stacks[self.end].push_front(c.clone());
            stacks[self.start].pop_front();
        }

        stacks
    }
}

impl FromStr for Move {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut m = Self::default();

        for (i, n) in s.split(" ").enumerate() {
            match n.parse::<usize>() {
                Ok(n) => match i {
                    1 => m.amount = n,
                    3 => m.start = n - 1,
                    5 => m.end = n - 1,
                    _ => {}
                },
                Err(_) => {}
            }
        }

        Ok(m)
    }
}

fn parse_stacks(sections: &Vec<&str>) -> Vec<VecDeque<String>> {
    let crate_section = sections[0].split("\n").collect::<Vec<&str>>();

    let mut stacks: Vec<VecDeque<String>> = Vec::new();
    let mut letter_indices: Vec<usize> = Vec::new();

    // determine how many stacks there are and which indices have the letters
    for (i, c) in crate_section[&crate_section.len() - 1].char_indices() {
        match c.to_digit(10) {
            Some(_) => {
                stacks.push(VecDeque::new());
                letter_indices.push(i);
            }
            _ => {}
        }
    }

    // create the stacks
    for row in sections[0].split("\n") {
        let mut stack_index = 0;

        for (i, c) in row.char_indices() {
            if letter_indices.contains(&i) {
                if c.is_alphabetic() {
                    stacks[stack_index].push_back(c.to_string());
                }
                stack_index += 1;
            }
        }
    }

    stacks
}

fn parse_moves(sections: &Vec<&str>) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();

    for row in sections[1].split("\n") {
        match Move::from_str(row) {
            Ok(m) => moves.push(m),
            Err(_) => {}
        }
    }

    moves
}

fn get_top_crates(input: String, mode: MoveMode) -> String {
    let sections = input.split("\n\n").collect::<Vec<&str>>();
    let mut stacks = parse_stacks(&sections);

    for m in parse_moves(&sections) {
        m.apply_move(&mut stacks, mode.clone());
    }

    let mut top_crates = String::new();

    for s in stacks {
        top_crates.push(s[0].chars().next().unwrap());
    }

    top_crates
}

pub fn part_1(input: String) -> String {
    get_top_crates(input, MoveMode::OneByOne)
}

pub fn part_2(input: String) -> String {
    get_top_crates(input, MoveMode::All)
}

#[cfg(test)]
mod day_5_tests {
    use super::*;
    use utils::parse_input;

    #[test]
    fn part_1_example() {
        let content = parse_input("example.txt").unwrap();
        let result = part_1(content);
        assert_eq!(result.as_str(), "CMZ");
    }

    #[test]
    fn part_1_input() {
        let content = parse_input("input.txt").unwrap();
        let result = part_1(content);
        assert_eq!(result.as_str(), "LJSVLTWQM");
    }

    #[test]
    fn part_2_example() {
        let content = parse_input("example.txt").unwrap();
        let result = part_2(content);
        assert_eq!(result.as_str(), "MCD");
    }

    #[test]
    fn part_2_input() {
        let content = parse_input("input.txt").unwrap();
        let result = part_2(content);
        assert_eq!(result.as_str(), "BRQWDBBJM");
    }
}
