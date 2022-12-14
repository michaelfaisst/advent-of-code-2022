pub fn solve(input: &str) -> usize {
    let pairs: Vec<&str> = input.split("\n\n").collect();

    pairs.into_iter().enumerate().fold(0, |acc, (index, pair)| {
        let (left, right) = pair.split_once("\n").map(|test| test).unwrap();
        acc
    })
}
