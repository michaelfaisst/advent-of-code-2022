pub fn find_marker(input: &str, marker_size: usize) -> usize {
    let chars: Vec<char> = input.chars().collect();
    let result = chars.windows(marker_size).enumerate().find(|(_, chars)| {
        let mut vec = chars.to_vec();
        vec.sort();
        vec.dedup();
        return vec.len() == marker_size;
    });

    result.unwrap().0 + marker_size
}
