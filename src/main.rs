use aoc_2022::{read_file, solve};

mod day12;

fn main() {
    let input: String = read_file(12, false);

    solve(day12::a::solve, &input);
    solve(day12::b::solve, &input);
}
