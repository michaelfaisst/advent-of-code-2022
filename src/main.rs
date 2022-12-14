use aoc_2022::{read_file, solve};

mod day13;

fn main() {
    let input: String = read_file(13, false);

    solve(day13::a::solve, &input);
    solve(day13::b::solve, &input);
}
