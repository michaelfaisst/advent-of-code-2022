use crate::day13::utils::{generate_element, is_valid};

pub fn solve(input: &str) -> usize {
    let pairs: Vec<&str> = input.split("\n\n").collect();

    pairs.into_iter().enumerate().fold(0, |acc, (index, pair)| {
        let (left, right) = pair
            .split_once("\n")
            .map(|test| (generate_element(test.0), generate_element(test.1)))
            .unwrap();

        let valid = is_valid(left, right).unwrap();

        return if valid { acc + index + 1 } else { acc };
    })
}
