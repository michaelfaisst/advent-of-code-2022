#[derive(PartialEq, Clone, Copy)]
enum Sign {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(PartialEq, Clone, Copy)]
enum Outcome {
    Lose = 0,
    Draw = 3,
    Win = 6,
}

fn parse_sign(sign_str: &str) -> Sign {
    match sign_str {
        "A" | "X" => Sign::Rock,
        "B" | "Y" => Sign::Paper,
        "C" | "Z" => Sign::Scissors,
        _ => Sign::Rock,
    }
}

fn parse_outcome(outcome_str: &str) -> Outcome {
    match outcome_str {
        "X" => Outcome::Lose,
        "Y" => Outcome::Draw,
        "Z" => Outcome::Win,
        _ => Outcome::Draw,
    }
}

fn parse_line(line: &str) -> (Sign, Outcome) {
    let (enemy_sign_str, outcome_str) = line.split_once(' ').unwrap();
    return (parse_sign(enemy_sign_str), parse_outcome(outcome_str));
}

fn calculate_response(enemy_sign: Sign, outcome: Outcome) -> Sign {
    if outcome == Outcome::Draw {
        return enemy_sign;
    } else if outcome == Outcome::Win {
        return match enemy_sign {
            Sign::Rock => Sign::Paper,
            Sign::Paper => Sign::Scissors,
            Sign::Scissors => Sign::Rock,
        };
    } else {
        return match enemy_sign {
            Sign::Rock => Sign::Scissors,
            Sign::Paper => Sign::Rock,
            Sign::Scissors => Sign::Paper,
        };
    }
}

pub fn solve(input: &str) -> usize {
    let lines = input.lines();

    lines.fold(0, |acc, line| {
        let (enemy_sign, outcome) = parse_line(line);
        let response_sign = calculate_response(enemy_sign, outcome);

        return acc + response_sign as usize + outcome as usize;
    })
}
