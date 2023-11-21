fn build_matrix(input: String) -> Vec<Vec<usize>> {
    return input
        .trim()
        .split("\n")
        .map(|line| {
            return line
                .chars()
                .map(|n| {
                    return n.to_string().parse::<usize>().unwrap();
                })
                .collect::<Vec<usize>>();
        })
        .collect::<Vec<Vec<usize>>>();
}

pub fn part_1(input: String) -> usize {
    let matrix = build_matrix(input);

    let mut total = 0;
    for (i, row) in matrix.iter().enumerate() {
        // if the first or last row, the trees are visible
        if i == 0 || i == matrix.len() - 1 {
            total += row.len();
            continue;
        }

        let mut visible = false;

        for (j, tree) in row.iter().enumerate() {
            // first and last trees are visible
            if j == 0 || j == row.len() - 1 {
                total += 1;
                continue;
            }

            // left to right
            let trees_to_right = &row[(j + 1)..];
            for (k, t) in trees_to_right.iter().enumerate() {
                if t >= tree {
                    break;
                }

                // if we reach the end of the array add 1 to the total
                if k == trees_to_right.len() - 1 {
                    total += 1;
                    visible = true;
                }
            }

            if visible {
                visible = false;
                continue;
            }

            // right to left
            let trees_to_left = &row[0..j];
            for (k, t) in trees_to_left.iter().rev().enumerate() {
                if t >= tree {
                    break;
                }

                // if we reach the end of the array add 1 to the total
                if k == trees_to_left.len() - 1 {
                    total += 1;
                    visible = true;
                }
            }

            if visible {
                visible = false;
                continue;
            }

            // to top
            let rows_to_top = &matrix[..i];
            for (k, row) in rows_to_top.iter().rev().enumerate() {
                if &row[j] >= tree {
                    break;
                }

                if k == rows_to_top.len() - 1 {
                    total += 1;
                    visible = true;
                }
            }

            if visible {
                visible = false;
                continue;
            }

            // to bottom
            let rows_to_bottom = &matrix[(i + 1)..];
            for (k, row) in rows_to_bottom.iter().enumerate() {
                if &row[j] >= tree {
                    break;
                }

                if k == rows_to_bottom.len() - 1 {
                    total += 1;
                    visible = true;
                }
            }

            if visible {
                visible = false;
                continue;
            }
        }
    }

    return total;
}

pub fn part_2(input: String) -> usize {
    let matrix = build_matrix(input);

    let mut total = 0;

    for (i, row) in matrix.iter().enumerate() {
        // ignore edges because multiplying by 0 = 0
        if i == 0 || i == matrix.len() - 1 {
            continue;
        }

        let mut tree_total: Vec<usize> = Vec::new();
        for (j, tree) in row.iter().enumerate() {
            // ignore edges because multiplying by 0 = 0
            if j == 0 || j == row.len() - 1 {
                continue;
            }

            // left to right
            let trees_to_right = &row[j + 1..];
            for (k, t) in trees_to_right.iter().enumerate() {
                if t >= tree && k == 0 {
                    tree_total.push(0);
                    break;
                }

                if t >= tree || k == trees_to_right.len() - 1 {
                    tree_total.push(k + 1);
                    break;
                }
            }

            // right to left
            let trees_to_left = &row[0..j];
            for (k, t) in trees_to_left.iter().rev().enumerate() {
                if t >= tree && k == 0 {
                    tree_total.push(0);
                    break;
                }

                if t >= tree || k == trees_to_left.len() - 1 {
                    tree_total.push(k + 1);
                    break;
                }
            }

            // to top
            let rows_to_top = &matrix[..i];
            for (k, row) in rows_to_top.iter().rev().enumerate() {
                if &row[j] >= tree && k == 0 {
                    tree_total.push(0);
                    break;
                }

                if &row[j] >= tree || k == rows_to_top.len() - 1 {
                    tree_total.push(k + 1);
                    break;
                }
            }

            // to bottom
            let rows_to_bottom = &matrix[(i + 1)..];
            for (k, row) in rows_to_bottom.iter().enumerate() {
                if &row[j] >= tree && k == 0 {
                    tree_total.push(0);
                    break;
                }

                if &row[j] >= tree || k == rows_to_bottom.len() - 1 {
                    tree_total.push(k + 1);
                    break;
                }
            }

            let product = tree_total.clone().iter().product(); 
            if product > total {
                total = product;
            }

            tree_total = Vec::new();
        }

    }

    return total;
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::parse_input;

    #[test]
    fn part1_example() {
        let content = parse_input("example.txt").unwrap();
        let result = part_1(content);
        assert_eq!(result, 21);
    }

    #[test]
    fn part1_input() {
        let content = parse_input("input.txt").unwrap();
        let result = part_1(content);
        assert_eq!(result, 1776);
    }

    #[test]
    fn part2_example() {
        let content = parse_input("example.txt").unwrap();
        let result = part_2(content);
        assert_eq!(result, 8);
    }

    #[test]
    fn part2_input() {
        let content = parse_input("input.txt").unwrap();
        let result = part_2(content);
        assert_eq!(result, 234416);
    }
}
