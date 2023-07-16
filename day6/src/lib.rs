use std::collections::HashSet;

fn get_marker(input: String, marker_length: usize) -> usize {
    let mut cursor = 0;

    while cursor <= input.len() - marker_length {
        let set = &input[cursor..cursor + marker_length]
            .chars()
            .collect::<HashSet<char>>();
        let set_len = set.len();

        match set_len {
            set_len if set_len < marker_length => {
                cursor += 1;
            }
            _ => break,
        }
    }

    cursor + marker_length
}

pub fn part_1(input: String) -> usize {
    get_marker(input, 4)
}

pub fn part_2(input: String) -> usize {
    get_marker(input, 14)
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::parse_input;

    #[test]
    fn part_1_example() {
        let content = parse_input("example_part1.txt").unwrap();
        let result = part_1(content);
        assert_eq!(result, 11);
    }

    #[test]
    fn part_1_input() {
        let content = parse_input("input.txt").unwrap();
        let result = part_1(content);
        assert_eq!(result, 1651);
    }

    #[test]
    fn part_2_example() {
        let content = parse_input("example_part2.txt").unwrap();
        let result = part_2(content);
        assert_eq!(result, 26);
    }

    #[test]
    fn part_2_input() {
        let content = parse_input("input.txt").unwrap();
        let result = part_2(content);
        assert_eq!(result, 3837);
    }
}
