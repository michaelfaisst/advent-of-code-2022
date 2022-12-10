pub fn solve(input: &str) -> usize {
    let lines = input.lines().into_iter().collect::<Vec<_>>();

    let numbers = lines
        .chunks(3)
        .map(|chunk| {
            let v_chunk = chunk.to_vec();
            let char = v_chunk
                .get(0)
                .unwrap()
                .chars()
                .find(|char| v_chunk[1].contains(*char) && v_chunk[2].contains(*char))
                .unwrap() as u8;

            return char - if char.is_ascii_lowercase() { 96 } else { 38 };
        })
        .map(|x| x as usize)
        .collect::<Vec<usize>>();

    return numbers.iter().sum();
}
