use std::collections::HashSet;

use super::utils::{parse_input, Pos};

pub fn solve(input: &str) -> usize {
    let (sensors, beacons) = parse_input(input);
    let y = 2_000_000;
    let mut observed_positions = HashSet::new();

    let min_x = sensors
        .iter()
        .map(|sensor| sensor.pos.x - sensor.max_distance)
        .min()
        .unwrap();
    let max_x = sensors
        .iter()
        .map(|sensor| sensor.pos.x + sensor.max_distance)
        .max()
        .unwrap();

    for i in min_x..max_x + 1 {
        let pos = Pos { x: i, y };
        let observed = sensors.iter().any(|sensor| sensor.is_pos_in_range(&pos));

        if observed {
            observed_positions.insert(pos);
        }
    }

    observed_positions.len() - beacons.into_iter().filter(|beacon| beacon.y == y).count()
}
