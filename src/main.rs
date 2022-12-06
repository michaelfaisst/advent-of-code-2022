use aoc_2022::{read_file, solve};

mod day6;

fn main() {
    let input: String = read_file(6, false);

    solve(day6::a::solve, &input);
    solve(day6::b::solve, &input);
}
