fn execute_command(
    command: &str,
    stack: &mut Vec<(String, usize)>,
    directories: &mut Vec<(String, usize)>,
) {
    let parts: Vec<&str> = command.split_whitespace().collect();

    match parts[1] {
        "cd" => match parts[2] {
            ".." => {
                let (name, size) = stack.pop().unwrap();
                stack.last_mut().unwrap().1 += size;
                directories.push((name, size));
            }
            value => {
                stack.push((value.to_string(), 0));
            }
        },
        "ls" => {}
        _ => eprintln!("Unknown command"),
    }
}

fn execute_ls_result(line: &str, stack: &mut Vec<(String, usize)>) {
    let parts: Vec<&str> = line.split_whitespace().collect();

    if !line.starts_with("dir") {
        stack.last_mut().unwrap().1 += parts[0].parse::<usize>().unwrap();
    } 
}

fn execute(
    line: &str,
    stack: &mut Vec<(String, usize)>,
    directories: &mut Vec<(String, usize)>,
) {
    if line.starts_with("$") {
        execute_command(line, stack, directories);
    } else {
        execute_ls_result(line, stack);
    }
}

pub fn calculate_directories(input: &str, directories: &mut Vec<(String, usize)>) {
    let mut line_iterator = input.lines().into_iter();
    let mut stack: Vec<(String, usize)> = vec![];
    
    let mut line = line_iterator.next();
    while line.is_some() {
        execute(line.unwrap(), &mut stack, directories);
        line = line_iterator.next();
    }

    while stack.len() > 0 {
        let (name, size) = stack.pop().unwrap();
        directories.push((name, size));

        if stack.len() > 0 {
            stack.last_mut().unwrap().1 += size;
        }
    }
}
