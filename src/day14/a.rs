use std::collections::HashSet;

use super::utils::{generate_rock_positions, get_max_y, simulate_sand_unit, Pos};

pub fn solve(input: &str) -> usize {
    let mut obstacles: HashSet<Pos> = generate_rock_positions(input);
    let max_y = get_max_y(&obstacles);

    let mut sand_count = 0;
    loop {
        let steady = simulate_sand_unit(&mut obstacles, Some(max_y));

        if !steady {
            break;
        }

        sand_count += 1;
    }

    sand_count
}
