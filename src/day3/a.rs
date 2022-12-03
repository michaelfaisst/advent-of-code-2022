pub fn solve(input: &str) -> usize {
    let lines = input.lines().map(|line| {
        let (left, right) = line.split_at(line.len() / 2);
        let found_char = left.chars().into_iter().find(|char| right.contains(*char)).unwrap() as u8;

        return found_char - if found_char.is_ascii_lowercase() { 96 } else { 38 }; 
    }).map(|x| x as usize).collect::<Vec<usize>>();

    return lines.iter().sum();
}
