use regex::Regex;
use std::{str::FromStr, string::ParseError};

#[derive(Debug)]
pub struct Instruction {
    amount: usize,
    from: usize,
    to: usize,
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"^move (\d*) from (\d*) to (\d*)$").unwrap();
        let matches = re.captures(s).unwrap();

        return Ok(Instruction {
            amount: matches.get(1).unwrap().as_str().parse().unwrap(),
            from: matches.get(2).unwrap().as_str().parse().unwrap(),
            to: matches.get(3).unwrap().as_str().parse().unwrap(),
        });
    }
}

pub fn create_stacks(input: &str, stacks: &mut Vec<Vec<char>>) {
    let lines = input.lines().rev().skip(1).collect::<Vec<&str>>();

    lines.into_iter().for_each(|line| {
        line.chars()
            .collect::<Vec<char>>()
            .chunks(4)
            .into_iter()
            .enumerate()
            .for_each(|(index, chunk)| {
                if chunk[1] != ' ' {
                    if stacks.get(index).is_none() {
                        stacks.push(Vec::new());
                    }

                    stacks[index].push(chunk[1]);
                }
            });
    });
}

pub fn execute_instruction(stacks: &mut Vec<Vec<char>>, instruction: &Instruction) {
    for _i in 0..instruction.amount {
        let value = stacks[instruction.from - 1].pop().unwrap();
        stacks[instruction.to - 1].push(value);
    }
}

pub fn execute_instruction_2(stacks: &mut Vec<Vec<char>>, instruction: &Instruction) {
    let from_index = stacks[instruction.from - 1].len() - instruction.amount;

    let mut removed = stacks[instruction.from - 1].drain(from_index..).collect::<Vec<char>>();
    stacks[instruction.to - 1].append(&mut removed);
}
