use std::{collections::HashSet, str::FromStr};

use super::utils::{move_head, update_knot, Pos, Step};

pub fn solve(input: &str) -> usize {
    let mut head: Pos = (0, 0);
    let mut tail: Pos = (0, 0);

    let mut visited_positions: HashSet<(isize, isize)> = HashSet::new();
    visited_positions.insert((0, 0));

    let steps = input
        .lines()
        .map(|line| Step::from_str(line).unwrap())
        .collect::<Vec<Step>>();

    for step in steps {
        for _ in 0..step.amount {
            move_head(&step, &mut head);
            tail = update_knot(tail, head);
            visited_positions.insert(tail);
        }
    }

    visited_positions.len()
}
