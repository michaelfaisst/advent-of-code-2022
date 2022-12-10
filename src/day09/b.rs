use std::{collections::HashSet, str::FromStr};

use super::utils::{move_head, update_knot, Pos, Step};

pub fn solve(input: &str) -> usize {
    let mut head: Pos = (0, 0);
    let mut knots: [Pos; 9] = [(0, 0); 9];

    let mut visited_positions: HashSet<(isize, isize)> = HashSet::new();
    visited_positions.insert((0, 0));

    let steps = input
        .lines()
        .map(|line| Step::from_str(line).unwrap())
        .collect::<Vec<Step>>();

    for step in steps {
        for _ in 0..step.amount {
            move_head(&step, &mut head);

            for i in 0..knots.len() {
                let reference = if i == 0 { head } else { knots[i - 1] };

                knots[i] = update_knot(knots[i], reference);

                if i == knots.len() - 1 {
                    visited_positions.insert(knots[i]);
                }
            }
        }
    }

    visited_positions.len()
}
