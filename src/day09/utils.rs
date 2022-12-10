use std::{str::FromStr, string::ParseError};

pub type Pos = (isize, isize);

#[derive(Debug)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug)]
pub struct Step {
    pub direction: Direction,
    pub amount: usize,
}

impl FromStr for Step {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_once(" ").unwrap();

        let direction = match left {
            "L" => Direction::Left,
            "R" => Direction::Right,
            "U" => Direction::Up,
            "D" => Direction::Down,
            _ => Direction::Left,
        };

        let amount = right.parse::<usize>().unwrap();

        Ok(Step { direction, amount })
    }
}

fn are_points_touching(p1: Pos, p2: Pos) -> bool {
    return p1.0.abs_diff(p2.0) <= 1 && p1.1.abs_diff(p2.1) <= 1;
}

fn move_knot(mut knot: Pos, reference: Pos) -> Pos {
    let delta = (reference.0 - knot.0, reference.1 - knot.1);
    let knot_step = (delta.0.clamp(-1, 1), delta.1.clamp(-1, 1));

    knot.0 += knot_step.0;
    knot.1 += knot_step.1;

    knot
}

pub fn move_head(step: &Step, head: &mut Pos) {
    match step.direction {
        Direction::Left => head.0 -= 1,
        Direction::Right => head.0 += 1,
        Direction::Up => head.1 += 1,
        Direction::Down => head.1 -= 1,
    }
}

pub fn update_knot(mut knot: Pos, reference: Pos) -> Pos {
    if !are_points_touching(knot, reference) {
        knot = move_knot(knot, reference);
    }

    knot
}
