use super::utils::Assignment;
use std::str::FromStr;

pub fn solve(input: &str) -> usize {
    input.lines().fold(0, |acc, line| {
        let (first, second) = line.split_once(",").unwrap();
        let first_assignment = Assignment::from_str(first).unwrap();
        let second_assignment = Assignment::from_str(second).unwrap();

        let overlaps = first_assignment.overlaps(&second_assignment)
            || second_assignment.overlaps(&first_assignment);

        acc + if overlaps == true { 1 } else { 0 }
    })
}
