use aoc_2022::{read_file, solve};

mod day5;

fn main() {
    let input: String = read_file(5, false);

    solve(day5::a::solve, &input);
    solve(day5::b::solve, &input);
}
