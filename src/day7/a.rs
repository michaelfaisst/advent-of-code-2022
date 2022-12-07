use super::utils::calculate_directories;

pub fn solve(input: &str) -> usize {
    let mut directories: Vec<(String, usize)> = vec![];
    calculate_directories(input, &mut directories);

    let result: usize = directories.iter().fold(0, |acc, (_, size)| {
        return if *size <= 100_000 { acc + size } else { acc };
    });

    result
}
