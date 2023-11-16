/// This solution is very similar if not the exact same as the primeagens.
///
/// see: https://github.com/ThePrimeagen/aoc/blob/2022/src/bin/day7.rs
///
/// I couldn't for the life of me figure this one out. I struggled with trying
/// to figure out how to organize the data for this one. Particularily rolling
/// the totals of the directories up from child directories. This was something
/// my brain literally couldnt comprehend.

const MAX_SIZE: usize = 100000;

const TOTAL_SPACE: usize = 70000000;
const DESIRED_SPACE: usize = 30000000;

pub fn puzzle(input: String) -> (usize, usize) {
    let mut stack: Vec<(&str, usize)> = vec![("/", 0)];
    let mut dir_totals = Vec::new();
    let mut total = 0;

    for line in input.trim().split("\n") {
        if line == "$ cd /" || line == "$ ls" {
            continue;
        }

        if line.starts_with("$ cd") {
            let dir_name = &line[5..];

            if dir_name == ".." {
                let dir = stack.pop().unwrap();

                if dir.1 <= MAX_SIZE {
                    total += dir.1;
                }

                stack.last_mut().unwrap().1 += dir.1;
                dir_totals.push(dir.1);
            } else {
                stack.push((dir_name, 0))
            }
            continue;
        }

        if let Ok(file_size) = line.split(" ").collect::<Vec<&str>>()[0].parse::<usize>() {
            stack.last_mut().unwrap().1 += file_size;
        } else {
            // don't care about this
        }
    }

    while stack.len() > 0 {
        let dir = stack.pop().unwrap();
        dir_totals.push(dir.1);

        if stack.len() > 0 {
            stack.last_mut().unwrap().1 += dir.1;
        }
    }

    let total_size = dir_totals.last().unwrap();
    let unused_space = TOTAL_SPACE - total_size;
    let desired_space = DESIRED_SPACE - unused_space;

    let dir_to_delete = dir_totals
        .into_iter()
        .filter(move |d| {
            return *d >= desired_space;
        })
        .min()
        .unwrap();

    return (total, dir_to_delete);
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::parse_input;

    #[test]
    fn puzzle_example() {
        let content = parse_input("example_part1.txt").unwrap();
        let result = puzzle(content);
        assert_eq!(result, (95437, 24933642));
    }

    #[test]
    fn puzzle_input() {
        let content = parse_input("input.txt").unwrap();
        let result = puzzle(content);
        assert_eq!(result, (1427048, 2940614));
    }
}
