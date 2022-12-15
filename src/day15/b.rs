use super::utils::{parse_input, Pos};
use std::cmp;

pub fn solve(input: &str) -> isize {
    let (sensors, _) = parse_input(input);
    let max_pos = 4_000_000;

    let mut x = 0;

    while x <= max_pos {
        let mut y = 0;

        while y <= max_pos {
            let pos = Pos { x, y };
            let sensor = sensors.iter().find(|sensor| sensor.is_pos_in_range(&pos));

            match sensor {
                Some(sensor) => y += cmp::max(sensor.max_distance - sensor.distance_from(&pos), 1),
                None => return pos.x * 4_000_000 + pos.y
            };
        }
        x += 1;
    }

    return 0;
}
