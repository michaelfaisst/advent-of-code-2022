use std::{env, fs};

pub fn read_file(day: u8, use_test_input: bool) -> String {
    let cwd = env::current_dir().unwrap();
    let mut file_name = format!("day{}.input", day);

    if use_test_input {
        file_name.push_str(".test");
    }

    let file_path = cwd.join("inputs").join(file_name);

    let file = fs::read_to_string(file_path);
    file.expect("could not read file")
}
