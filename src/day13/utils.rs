use std::{iter::zip, str::Chars};

#[derive(Debug)]
pub enum Element {
    Number(u32),
    Elem(Vec<Element>),
}

fn parse_array(iter: &mut Chars) -> Vec<Element> {
    let mut vec: Vec<Element> = vec![];
    let mut num_str: String = "".to_string();

    loop {
        let char = iter.next().unwrap();

        if !char.is_digit(10) && num_str.len() > 0 {
            let num = num_str.parse::<u32>().unwrap();
            vec.push(Element::Number(num));
            num_str.clear();
        }

        if char == '[' {
            vec.push(Element::Elem(parse_array(iter)));
        } else if char == ']' {
            return vec;
        } else if char.is_digit(10) {
            num_str.push(char);
        }
    }
}

pub fn generate_elements(input: &str) -> Vec<Element> {
    let mut iter = input.chars().into_iter();
    iter.next(); // skip the first [ because its not needed
    parse_array(&mut iter)
}

pub fn compare_elements(a_vec: Vec<Element>, b_vec: Vec<Element>) -> Option<bool> {
    let lengths = (a_vec.len(), b_vec.len());

    for (a, b) in zip(a_vec.into_iter(), b_vec.into_iter()) {
        match is_valid(a, b) {
            Some(valid) => return Some(valid),
            None => continue,
        }
    }

    if lengths.0 < lengths.1 {
        Some(true)
    } else if lengths.0 > lengths.1 {
        Some(false)
    } else {
        None
    }
}

pub fn is_valid(a: Element, b: Element) -> Option<bool> {
    match a {
        Element::Number(a_num) => match b {
            Element::Number(b_num) => {
                if a_num < b_num {
                    Some(true)
                } else if a_num > b_num {
                    Some(false)
                } else {
                    None
                }
            }
            Element::Elem(b_vec) => compare_elements(vec![a], b_vec),
        },
        Element::Elem(a_vec) => match b {
            Element::Number(_) => compare_elements(a_vec, vec![b]),
            Element::Elem(b_vec) => compare_elements(a_vec, b_vec),
        },
    }
}
