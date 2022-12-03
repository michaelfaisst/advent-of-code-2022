use aoc_2022::{read_file, solve};

mod day3;

fn main() {
    let input: String = read_file(3, false);

    solve(day3::a::solve, &input);
    // solve(day2::b::solve, &input);
}
