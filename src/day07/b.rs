use super::utils::calculate_directories;

pub fn solve(input: &str) -> usize {
    let mut directories: Vec<(String, usize)> = vec![];
    calculate_directories(input, &mut directories);

    let mut directory_sizes: Vec<usize> = directories.iter().map(|dir| dir.1).collect();
    directory_sizes.sort_by(|a, b| b.cmp(a));

    let needed_space = 30_000_000 - (70_000_000 - directory_sizes[0]);

    directory_sizes.into_iter().filter(|size| *size > needed_space).min().unwrap()
}
