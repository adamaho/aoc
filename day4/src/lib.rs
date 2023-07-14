#[derive(Debug)]
pub struct Assignment {
    start: usize,
    end: usize,
}

pub fn parse_pairs(input: String) -> Vec<Vec<Assignment>> {
    input
        .split("\n")
        .map(|r| {
            r.split(",")
                .map(|s| {
                    let items = s
                        .split("-")
                        .map(|n| n.parse::<usize>().unwrap_or(0))
                        .collect::<Vec<usize>>();

                    Assignment {
                        start: items[0],
                        end: items[1],
                    }
                })
                .collect::<Vec<Assignment>>()
        })
        .collect::<Vec<Vec<Assignment>>>()
}

fn is_contained(first: &Assignment, second: &Assignment) -> usize {
    if first.start >= second.start && first.end <= second.end {
        return 1;
    }

    if second.start >= first.start && second.end <= first.end {
        return 1;
    }

    return 0;
}

fn is_overlapping(first: &Assignment, second: &Assignment) -> usize {
    if first.end >= second.start && first.start <= second.end {
        return 1;
    }

    return 0;
}

pub fn part_1(input: String) -> usize {
    parse_pairs(input)
        .iter()
        .map(|p| {
            return is_contained(&p[0], &p[1]);
        })
        .sum::<usize>()
}

pub fn part_2(input: String) -> usize {
    parse_pairs(input)
        .iter()
        .map(|p| {
            return is_overlapping(&p[0], &p[1]);
        })
        .sum::<usize>()
}

#[cfg(test)]
mod day_4_tests {
    use super::*;
    use utils::parse_input;

    #[test]
    fn part_1_example() {
        let content = parse_input("example.txt").unwrap();
        let result = part_1(content);
        assert_eq!(result, 2);
    }

    #[test]
    fn part_1_input() {
        let content = parse_input("input.txt").unwrap();
        let result = part_1(content);
        assert_eq!(result, 500);
    }

    #[test]
    fn part_2_example() {
        let content = parse_input("example.txt").unwrap();
        let result = part_2(content);
        assert_eq!(result, 4);
    }

    #[test]
    fn part_2_input() {
        let content = parse_input("input.txt").unwrap();
        let result = part_2(content);
        assert_eq!(result, 815);
    }
}
