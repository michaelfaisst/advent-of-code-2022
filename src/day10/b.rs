use std::collections::HashMap;

use super::utils::execute_instruction;

pub fn solve(input: &str) -> isize {
    let instructions = input.lines();
    let mut crt = "".to_string();

    let mut cycle = 1;
    let mut cycles = HashMap::new();
    let mut x: isize = 1;

    instructions.into_iter().for_each(|instruction| {
        execute_instruction(instruction, &mut cycle, &mut x, &mut cycles, &mut crt)
    });

    crt.chars().collect::<Vec<char>>().chunks(40).for_each(|chunk| {
        println!("{}", chunk.iter().collect::<String>());
    });

    0
}
