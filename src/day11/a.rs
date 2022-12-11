use std::str::FromStr;

use super::utils::Monkey;

pub fn solve(input: &str) -> usize {
    let mut monkeys: Vec<Monkey> = input
        .split("\n\n")
        .map(|block| Monkey::from_str(block).unwrap())
        .collect();

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            while monkeys[i].items.len() > 0 {
                let (next_monkey, value) =
                    monkeys[i].execture_turn(true).expect("should never happen");
                monkeys[next_monkey].items.push_back(value);
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
