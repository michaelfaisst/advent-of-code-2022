use crate::day13::utils::{generate_element, is_valid};
use std::cmp::Ordering;

use super::utils::Element;

pub fn solve(input: &str) -> usize {
    let mut elements: Vec<Element> = input
        .replace("\n\n", "\n")
        .lines()
        .map(|line| generate_element(line))
        .collect();

    let divider_1 = Element::List(vec![Element::List(vec![Element::Number(2)])]);
    let divider_2 = Element::List(vec![Element::List(vec![Element::Number(6)])]);

    elements.push(divider_1.clone());
    elements.push(divider_2.clone());

    elements.sort_by(|a, b| {
        let valid = is_valid(a.to_owned(), b.to_owned()).unwrap();

        return if valid {
            Ordering::Less
        } else {
            Ordering::Greater
        };
    });

    elements
        .into_iter()
        .enumerate()
        .filter(|(_, elem)| elem == &divider_1 || elem == &divider_2)
        .map(|(index, _)| index + 1)
        .product()
}
