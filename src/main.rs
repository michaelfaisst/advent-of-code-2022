use aoc_2022::{read_file, solve};

mod day2;

fn main() {
    let input: String = read_file(2, false);

    solve(day2::a::solve, &input);
    solve(day2::b::solve, &input);
}
