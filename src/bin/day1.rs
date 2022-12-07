fn main() {
    let input = std::fs::read_to_string("src/bin/day1-input.txt").unwrap();

    let mut calories: Vec<i32> = input
        .split("\n\n")
        .map(|elf| {
            return elf
                .split("\n")
                .map(|cals| match cals.parse::<i32>() {
                    Ok(cal) => cal,
                    Err(_) => 0,
                })
                .sum::<i32>();
        })
        .collect::<Vec<i32>>();

    calories.sort_by(|a, b| b.cmp(a));

    let sum_of_top_3 = &calories[0..3].iter().sum::<i32>();

    println!("Part 1: {:?}", calories[0]);
    println!("Part 2: {:?}", sum_of_top_3);
}
