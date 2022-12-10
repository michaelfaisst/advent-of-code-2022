use aoc_2022::{read_file, solve};

mod day9;

fn main() {
    let input: String = read_file(9, false);

    solve(day9::a::solve, &input);
    solve(day9::b::solve, &input);
}
