use crate::day13::utils::{compare_elements, generate_elements};

pub fn solve(input: &str) -> usize {
    let pairs: Vec<&str> = input.split("\n\n").collect();

    pairs.into_iter().enumerate().fold(0, |acc, (index, pair)| {
        let (left, right) = pair
            .split_once("\n")
            .map(|test| (generate_elements(test.0), generate_elements(test.1)))
            .unwrap();

        println!("{:?}", left);

        let valid = compare_elements(left, right).unwrap();

        return if valid { acc + index + 1 } else { acc };
    })
}
