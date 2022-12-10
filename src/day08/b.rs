use super::utils::{fill_grid, Grid};

fn calculate_scenic_score(grid: &Grid, row_index: usize, col_index: usize) -> usize {
    let value = grid[row_index][col_index];

    let column = grid[..]
        .iter()
        .map(|row| row[col_index])
        .collect::<Vec<u32>>();

    let slices = [
        &mut grid[row_index][..col_index].to_vec(),
        &mut grid[row_index][col_index + 1..].to_vec(),
        &mut column[..row_index].to_vec(),
        &mut column[row_index + 1..].to_vec(),
    ];

    slices[0].reverse();
    slices[2].reverse();

    slices.into_iter().map(|slice| {
        let mut distance = 0;
        for elem in slice {
            distance += 1;

            if *elem >= value {
                break;
            }
        }         

        distance
    }).product()
}

fn count_visible(grid: &Grid) -> usize {
    let mut high_score = 0;

    for (row_index, row) in grid.iter().enumerate() {
        for (col_index, _) in row.iter().enumerate() {
            let score = calculate_scenic_score(&grid, row_index, col_index);

            if score > high_score {
                high_score = score;
            }
        }
    }

    high_score
}

pub fn solve(input: &str) -> usize {
    let mut grid: Grid = vec![];

    fill_grid(input, &mut grid);
    count_visible(&grid)
}
