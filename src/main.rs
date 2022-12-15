use aoc_2022::{read_file, solve};

mod day15;

fn main() {
    let input: String = read_file(15, false);

    // solve(day15::a::solve, &input);
    solve(day15::b::solve, &input);
}
