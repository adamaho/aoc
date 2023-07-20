use std::{collections::HashSet, str::FromStr};

#[derive(Debug)]
struct FileSystem {
    size: usize,
}

pub fn part_1(input: String) -> usize {
    for line in input.split("\n") {
        let l = line.split(" ").collect::<Vec<&str>>();

        match l[0] {
            "$" => match l[1] {
                "cd" => {
                    println!("change dir");
                }
                "ls" => {
                    println!("list dir");
                }
                _ => {
                    unreachable!("Unknown command: {}", l[1]);
                }
            },
            "dir" => {
                println!("dir")
            }
            _ => {
                println!("file")
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::parse_input;

    #[test]
    fn part_1_example() {
        let content = parse_input("example.txt").unwrap();
        let result = part_1(content);
        assert_eq!(result, 0);
    }
}
