/// Sets the input data up in a format that is easy to work with
/// for both parts
pub fn format_input(input: String) -> Vec<usize> {
    input
        .split("\n\n")
        .map(|e| {
            e.split("\n")
                .map(|c| c.parse::<usize>().unwrap_or_default())
                .sum::<usize>()
        })
        .collect::<Vec<usize>>()
}

/// Computes the answer for part 1
pub fn part_1(input: String) -> usize {
    format_input(input).into_iter().max().unwrap_or_default()
}

/// Computes the answer for part 2
pub fn part_2(input: String) -> usize {
    let mut cals = format_input(input);
    cals.sort();
    cals.reverse();
    cals[0..=2].iter().sum::<usize>()
}

#[cfg(test)]
mod day_1_test {
    use super::*;
    use utils::parse_input;

    #[test]
    fn part_1_example() {
        let content = parse_input("example.txt").unwrap();
        let result = part_1(content);
        assert_eq!(result, 24000);
    }

    #[test]
    fn part_1_input() {
        let content = parse_input("input.txt").unwrap();
        let result = part_1(content);
        assert_eq!(result, 67633);
    }

    #[test]
    fn part_2_example() {
        let content = parse_input("example.txt").unwrap();
        let result = part_2(content);
        println!("{:?}", result);
        assert_eq!(result, 45000);
    }

    #[test]
    fn part_2_input() {
        let content = parse_input("input.txt").unwrap();
        let result = part_2(content);
        assert_eq!(result, 199628);
    }
}
