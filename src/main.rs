use aoc_2022::{read_file, solve};

mod day1;

fn main() {
    let input: String = read_file(1, false);

    solve(day1::part_1, &input);
    solve(day1::part_2, &input);
}
