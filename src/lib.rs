use std::{env, fmt::Display, fs, time::Instant};

pub fn read_file(day: u8, use_test_input: bool) -> String {
    let cwd = env::current_dir().unwrap();
    let mut file_name = format!("day{:0>2}.input", day);

    if use_test_input {
        file_name.push_str(".test");
    }

    let file_path = cwd.join("src").join("inputs").join(file_name);

    let file = fs::read_to_string(file_path);
    file.expect("could not read file")
}

pub fn solve<T: Display>(func: impl Fn(&str) -> T, input: &str) {
    let timer = Instant::now();
    let result: T = func(input);
    let elapsed = timer.elapsed();

    println!("{} (elapsed: {:.2?})", result, elapsed)
}
