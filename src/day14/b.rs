use std::collections::HashSet;

use super::utils::{generate_rock_positions, get_max_y, simulate_sand_unit, Pos};

pub fn solve(input: &str) -> usize {
    let mut obstacles: HashSet<Pos> = generate_rock_positions(input);
    let max_y = get_max_y(&obstacles);

    // jesus, I'm lazy and could add some checks to check for the floor
    // but I'm just gonna add the floor in manually :D
    for i in 0..1000 {
        obstacles.insert(Pos { x: i, y: max_y + 2 });
    }

    let mut sand_count = 0;
    loop {
        sand_count += 1;
        simulate_sand_unit(&mut obstacles, None);

        if obstacles.contains(&Pos { x: 500, y: 0 }) {
            break;
        }
    }

    sand_count
}
