pub type Grid = Vec<Vec<u32>>;

pub fn fill_grid(input: &str, grid: &mut Grid) {
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
