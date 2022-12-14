use super::utils::{find_all_pos, find_pos, find_shortest_path};

pub fn solve(input: &str) -> usize {
    let cleaned_input = input.replace("S", "a");
    let grid: Vec<Vec<char>> = cleaned_input
        .lines()
        .into_iter()
        .map(|line| {
            let test: Vec<char> = line.replace("E", "z").chars().collect();
            return test;
        })
        .collect();

    let possible_start_positions = find_all_pos('a', &cleaned_input, &grid);
    let end_pos = find_pos("E", input, &grid);

    possible_start_positions.into_iter().flat_map(|start_pos| {
        find_shortest_path(start_pos, end_pos, &grid)
    }).map(|x| x.len() - 1).min().unwrap()
}
