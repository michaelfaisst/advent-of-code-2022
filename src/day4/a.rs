use super::utils::Assignment;
use std::str::FromStr;

pub fn solve(input: &str) -> usize {
    input.lines().fold(0, |acc, line| {
        let (first, second) = line.split_once(",").unwrap();
        let first_assignment = Assignment::from_str(first).unwrap();
        let second_assignment = Assignment::from_str(second).unwrap();

        let contains = first_assignment.contains_other_assignment(&second_assignment)
            || second_assignment.contains_other_assignment(&first_assignment);

        acc + if contains == true { 1 } else { 0 }
    })
}
