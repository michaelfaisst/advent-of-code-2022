use aoc_2022::{read_file, solve};

mod day8;

fn main() {
    let input: String = read_file(8, false);

    solve(day8::a::solve, &input);
    solve(day8::b::solve, &input);
}
