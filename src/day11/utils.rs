use std::{collections::VecDeque, str::FromStr, string::ParseError};

#[derive(Debug)]
pub enum OperationValue {
    Old,
    Value(usize),
}

#[derive(Debug)]
pub struct Monkey {
    pub items: VecDeque<usize>,
    pub operation: String,
    pub operation_value: OperationValue,
    pub test_value: usize,
    pub monkey_if_true: usize,
    pub monkey_if_false: usize,
    pub inspected_items: usize,
}

fn get_operation_value(input: &str) -> OperationValue {
    let value = input.split(" ").last().unwrap();

    if value == "old" {
        return OperationValue::Old;
    } else {
        return OperationValue::Value(value.parse().unwrap());
    }
}

fn get_last_value(input: &str) -> usize {
    input.split(" ").last().unwrap().parse::<usize>().unwrap()
}

impl FromStr for Monkey {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.split("\n").collect::<Vec<&str>>();

        let (_, items) = lines[1].split_once(": ").unwrap();
        let items = items
            .split(",")
            .into_iter()
            .map(|value| value.trim().parse::<usize>().unwrap())
            .collect::<VecDeque<usize>>();

        let operation = if lines[2].contains("*") { "*" } else { "+" };

        let operation_value = get_operation_value(lines[2]);
        let test_value = get_last_value(lines[3]);
        let monkey_if_true = get_last_value(lines[4]);
        let monkey_if_false = get_last_value(lines[5]);

        Ok(Monkey {
            items,
            operation: operation.to_string(),
            operation_value,
            test_value,
            monkey_if_true,
            monkey_if_false,
            inspected_items: 0,
        })
    }
}

impl Monkey {
    pub fn execture_turn(&mut self, divide_by_three: bool) -> Option<(usize, usize)> {
        if self.items.len() == 0 {
            return None;
        }

        let mut item = self.items.pop_front().unwrap();
        let operation_value = match self.operation_value {
            OperationValue::Old => item,
            OperationValue::Value(value) => value,
        };

        item = if self.operation == "+" {
            item + operation_value
        } else {
            item * operation_value
        };

        if divide_by_three {
            item = item / 3;
        }

        let next_monkey = if item % self.test_value == 0 {
            self.monkey_if_true
        } else {
            self.monkey_if_false
        };

        self.inspected_items += 1;

        return Some((next_monkey, item));
    }
}

pub fn get_monkey_business(input: &str, iterations: usize, relief: bool) -> usize {
    let mut monkeys: Vec<Monkey> = input
        .split("\n\n")
        .map(|block| Monkey::from_str(block).unwrap())
        .collect();

    let lcm = monkeys.iter().map(|monkey| monkey.test_value).product::<usize>();

    for _ in 0..iterations {
        for i in 0..monkeys.len() {
            while monkeys[i].items.len() > 0 {
                let (next_monkey, value) =
                    monkeys[i].execture_turn(relief).expect("should never happen");
                monkeys[next_monkey].items.push_back(value % lcm);
            }
        }
    }

    monkeys.sort_by(|a, b| b.inspected_items.cmp(&a.inspected_items));
    monkeys
        .iter()
        .take(2)
        .map(|monkey| monkey.inspected_items)
        .product::<usize>()
}
