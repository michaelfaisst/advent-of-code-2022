use aoc_2022::{read_file, solve};

mod day10;

fn main() {
    let input: String = read_file(10, false);

    solve(day10::a::solve, &input);
    solve(day10::b::solve, &input);
}
