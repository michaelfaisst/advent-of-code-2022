#[derive(PartialEq, Clone, Copy)]
enum Sign {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

fn parse_sign(sign_str: &str) -> Sign {
    match sign_str {
        "A" | "X" => Sign::Rock,
        "B" | "Y" => Sign::Paper,
        "C" | "Z" => Sign::Scissors,
        _ => Sign::Rock,
    }
}

fn parse_line(line: &str) -> (Sign, Sign) {
    let (enemy_sign_str, my_sign_str) = line.split_once(' ').unwrap();
    return (parse_sign(enemy_sign_str), parse_sign(my_sign_str));
}

fn calculate_winner(enemy_sign: Sign, my_sign: Sign) -> usize {
    if my_sign == enemy_sign {
        return 3;
    };

    if (my_sign == Sign::Rock && enemy_sign == Sign::Scissors)
        || (my_sign == Sign::Paper && enemy_sign == Sign::Rock)
        || (my_sign == Sign::Scissors && enemy_sign == Sign::Paper)
    {
        return 6;
    }

    return 0;
}

pub fn solve(input: &str) -> usize {
    let lines = input.lines();

    lines.fold(0, |acc, line| {
        let (enemy_sign, my_sign) = parse_line(line);
        return acc + calculate_winner(enemy_sign, my_sign) + my_sign as usize;
    })
}
