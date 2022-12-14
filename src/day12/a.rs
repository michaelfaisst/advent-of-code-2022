use super::utils::{find_pos, find_shortest_path};

pub fn solve(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input
        .lines()
        .into_iter()
        .map(|line| {
            let test: Vec<char> = line.replace("S", "a").replace("E", "z").chars().collect();
            return test;
        })
        .collect();

    let start_pos = find_pos("S", input, &grid);
    let end_pos = find_pos("E", input, &grid);

    let shortest_path = find_shortest_path(start_pos, end_pos, &grid);
    shortest_path.unwrap().len() - 1
}
