use std::collections::HashMap;

use super::utils::execute_instruction;

pub fn solve(input: &str) -> isize {
    let instructions = input.lines();
    let needed_cycles: [usize; 6] = [20, 60, 100, 140, 180, 220];
    let mut crt = "".to_string();

    let mut cycle = 1;
    let mut cycles = HashMap::new();
    let mut x: isize = 1;

    instructions.into_iter().for_each(|instruction| {
        execute_instruction(instruction, &mut cycle, &mut x, &mut cycles, &mut crt)
    });

    needed_cycles.into_iter().fold(0, |mut acc, cycle| {
        acc += cycles.get(&cycle).unwrap() * cycle as isize;
        acc
    })
}
