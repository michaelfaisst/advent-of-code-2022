use aoc_2022::{read_file, solve};

mod day7;

fn main() {
    let input: String = read_file(7, false);

    solve(day7::a::solve, &input);
    solve(day7::b::solve, &input);
}
