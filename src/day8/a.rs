type Grid = Vec<Vec<u32>>;

fn fill_grid(input: &str, grid: &mut Grid) {
    let lines: Vec<&str> = input.lines().collect();

    for line in lines {
        grid.push(
            line.chars()
                .into_iter()
                .map(|val| val.to_digit(10).unwrap())
                .collect(),
        );
    }
}

fn is_visible(grid: &Grid, row_index: usize, col_index: usize) -> bool {
    let value = grid[row_index][col_index];

    let column = grid[..].iter().map(|row| row[col_index]).collect::<Vec<u32>>();

    let slices = [
        &grid[row_index][..col_index],
        &grid[row_index][col_index + 1..],
        &column[..row_index],
        &column[row_index + 1..]
    ];

    slices.iter().any(|slice| {
        return slice.into_iter().all(|x| *x < value);
    })
}

fn count_visible(grid: &Grid) -> usize {
    let mut count = 0;

    count += 2 * (grid.len() + grid[0].len()) - 4;

    for (row_index, row) in grid[1..grid.len() - 1].iter().enumerate() {
        for (col_index, _) in row[1..row.len() - 1].iter().enumerate() {
            let visible = is_visible(&grid, row_index + 1, col_index + 1);
            count += if visible { 1 } else { 0 };
        }
    }

    count
}

pub fn solve(input: &str) -> usize {
    let mut grid: Grid = vec![];

    fill_grid(input, &mut grid);
    count_visible(&grid)
}
