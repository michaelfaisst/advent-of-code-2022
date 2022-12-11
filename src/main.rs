use aoc_2022::{read_file, solve};

mod day11;

fn main() {
    let input: String = read_file(11, false);

    solve(day11::a::solve, &input);
    solve(day11::b::solve, &input);
}
