use std::collections::HashMap;

pub fn update_crt(crt: &mut String, cycle: &usize, x: &isize) {
    let mut val = *cycle as isize;
    val -= 1;

    if val % 40 >= x - 1 && val % 40 <= x + 1 {
        crt.push('#');
    } else {
        crt.push('.')
    }
}

pub fn execute_instruction(
    instruction: &str,
    cycle: &mut usize,
    x: &mut isize,
    cycles: &mut HashMap<usize, isize>,
    crt: &mut String,
) {
    if instruction.starts_with("noop") {
        cycles.insert(*cycle, *x);
        update_crt(crt, cycle, x);
        *cycle += 1;
        return;
    }

    let (_, p) = instruction.split_once(" ").unwrap();
    let param: isize = p.parse().unwrap();

    for _ in 0..2 {
        cycles.insert(*cycle, *x);
        update_crt(crt, cycle, x);
        *cycle += 1;
    }

    *x += param;
}
