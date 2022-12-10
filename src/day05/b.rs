use std::str::FromStr;

use crate::day5::utils::{Instruction, execute_instruction_2};

use super::utils::create_stacks;

pub fn solve(input: &str) -> String {
    let mut stacks: Vec<Vec<char>> = Vec::new();
    let (start_schema, instruction_lines) = input.split_once("\n\n").unwrap();

    create_stacks(start_schema, &mut stacks);

    instruction_lines.lines().for_each(|line| {
        let instruction = Instruction::from_str(line).unwrap();
        execute_instruction_2(&mut stacks, &instruction);
    });

    let result = stacks.iter().fold("".to_string(), |acc, stack| {
        let char = stack.last().unwrap();
        return acc + &char.to_string();
    });

    return result;
}
