use std::collections::HashMap;

struct Priority;

impl Priority {
    // returns the priority for the given letter
    fn get_priority(s: &str) -> usize {
        match s {
            "a" => 1,
            "b" => 2,
            "c" => 3,
            "d" => 4,
            "e" => 5,
            "f" => 6,
            "g" => 7,
            "h" => 8,
            "i" => 9,
            "j" => 10,
            "k" => 11,
            "l" => 12,
            "m" => 13,
            "n" => 14,
            "o" => 15,
            "p" => 16,
            "q" => 17,
            "r" => 18,
            "s" => 19,
            "t" => 20,
            "u" => 21,
            "v" => 22,
            "w" => 23,
            "x" => 24,
            "y" => 25,
            "z" => 26,
            "A" => 27,
            "B" => 28,
            "C" => 29,
            "D" => 30,
            "E" => 31,
            "F" => 32,
            "G" => 33,
            "H" => 34,
            "I" => 35,
            "J" => 36,
            "K" => 37,
            "L" => 38,
            "M" => 39,
            "N" => 40,
            "O" => 41,
            "P" => 42,
            "Q" => 43,
            "R" => 44,
            "S" => 45,
            "T" => 46,
            "U" => 47,
            "V" => 48,
            "W" => 49,
            "X" => 50,
            "Y" => 51,
            "Z" => 52,
            _ => 1,
        }
    }
}

pub fn part_1(input: String) -> usize {
    input
        .split("\n")
        .map(|s| {
            let sack_vertex = (s.len() / 2) - 1;

            let mut chars: HashMap<String, (bool, bool)> = HashMap::new();

            for (i, c) in s.char_indices() {
                let key = c.to_string();
                let curr = chars.get(&key).cloned().unwrap_or((false, false));
                if i <= sack_vertex {
                    chars.insert(key.clone(), (true, curr.1));
                } else {
                    chars.insert(key, (curr.0, true));
                }
            }

            let mut char: String = String::new();

            for (key, value) in &chars {
                if value.to_owned() == (true, true) {
                    char = key.to_string();
                    break;
                }
            }

            Priority::get_priority(char.as_str())
        })
        .sum::<usize>()
}

pub fn part_2(input: String) -> usize {
    let mut groups: Vec<Vec<&str>> = Vec::new();
    let mut group = Vec::new();

    for (i, r) in input.split("\n").enumerate() {
        if i % 3 == 2 {
            group.push(r);
            groups.push(group);
            group = Vec::new();
        } else {
            group.push(r);
        }
    }

    groups
        .iter()
        .map(|group| {
            let mut chars: HashMap<String, (bool, bool, bool)> = HashMap::new();

            for (i, s) in group.iter().enumerate() {
                for c in s.chars() {
                    let key = c.to_string();
                    let curr = chars.get(&key).cloned().unwrap_or((false, false, false));

                    match i {
                        0 => {
                            chars.insert(key, (true, curr.1, curr.2));
                        }
                        1 => {
                            chars.insert(key, (curr.0, true, curr.2));
                        }
                        2 => {
                            chars.insert(key, (curr.0, curr.1, true));
                        }
                        _ => {
                            chars.insert(key, (false, false, false));
                        }
                    }
                }
            }

            let mut char: String = String::new();

            for (key, value) in &chars {
                if value.to_owned() == (true, true, true) {
                    char = key.to_string();
                    break;
                }
            }

            Priority::get_priority(char.as_str())
        })
        .sum::<usize>()
}

#[cfg(test)]
mod day_3_tests {
    use super::*;
    use utils::parse_input;

    #[test]
    fn part_1_example() {
        let content = parse_input("example.txt").unwrap();
        let result = part_1(content);
        assert_eq!(result, 157);
    }

    #[test]
    fn part_1_input() {
        let content = parse_input("input.txt").unwrap();
        let result = part_1(content);
        assert_eq!(result, 7795);
    }

    #[test]
    fn part_2_example() {
        let content = parse_input("example.txt").unwrap();
        let result = part_2(content);
        assert_eq!(result, 70);
    }

    #[test]
    fn part_2_input() {
        let content = parse_input("input.txt").unwrap();
        let result = part_2(content);
        assert_eq!(result, 2703);
    }
}
