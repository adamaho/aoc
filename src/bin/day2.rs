fn get_index_for_letter(letter: &str) -> usize {
    return match letter {
        "A" => 0,
        "B" => 1,
        "C" => 2,
        "X" => 0,
        "Y" => 1,
        "Z" => 2,
        _ => unreachable!("A unknown letter: {} was passed.", letter),
    };
}

fn get_score_for_move(letter: &str) -> i32 {
    return match letter {
        "A" => 1,
        "B" => 2,
        "C" => 3,
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => unreachable!("A unknown letter: {} was passed.", letter),
    };
}

fn get_score_for_outcome(letter: &str) -> i32 {
    return match letter {
        "X" => 0,
        "Y" => 3,
        "Z" => 6,
        _ => unreachable!("A unknown letter: {} was passed.", letter),
    };
}

fn main() {
    let input = std::fs::read_to_string("src/bin/day2.txt").unwrap();

    // Part 1
    let win_loss_matrix = vec![vec![1, 0, 2], vec![2, 1, 0], vec![0, 2, 1]];

    let part_1_score = input
        .split("\n")
        .map(|round| {
            let round_moves = round.split(" ").collect::<Vec<&str>>();

            let player1_index = get_index_for_letter(round_moves[0]);
            let player2_index = get_index_for_letter(round_moves[1]);

            return (win_loss_matrix[player2_index][player1_index] * 3)
                + get_score_for_move(round_moves[1]);
        })
        .sum::<i32>();

    // Part 2
    let outcome_matrix = win_loss_matrix
        .iter()
        .map(|row| row.into_iter().rev().collect::<Vec<&i32>>())
        .collect::<Vec<Vec<&i32>>>();

    let part_2_score = input
        .split("\n")
        .map(|round| {
            let round_moves = round.split(" ").collect::<Vec<&str>>();

            let player1_index = get_index_for_letter(round_moves[0]);
            let player2_index = get_index_for_letter(round_moves[1]);

            let outcome_score = get_score_for_outcome(round_moves[1]);

            let move_score = outcome_matrix[player1_index][player2_index] + 1;

            return move_score + outcome_score;
        })
        .sum::<i32>();

    print!("part 1 total score: {}\n", part_1_score);
    print!("part 2 total score: {}\n", part_2_score);
}
