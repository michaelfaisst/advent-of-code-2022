use aoc_2022::{read_file, solve};

mod day4;

fn main() {
    let input: String = read_file(4, false);

    solve(day4::a::solve, &input);
    solve(day4::b::solve, &input);
}
