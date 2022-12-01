use aoc_2022::read_file;

fn calc_total_calories(group: &str) -> usize {
    let calories: Vec<usize> = group.lines().map(|line| line.parse::<usize>().unwrap()).collect();
    calories.iter().sum::<usize>()
}

pub fn part_1() -> usize {
    let input = read_file(1, false);
    let input_groups = input.split("\n\n");

    let calories = input_groups.map(|group| calc_total_calories(group)).collect::<Vec<usize>>();
    *calories.iter().max().unwrap()
}

pub fn part_2() -> usize {
    let input = read_file(1, false);
    let input_groups = input.split("\n\n");

    let mut calories = input_groups.into_iter().map(|group| calc_total_calories(group)).collect::<Vec<usize>>();
    calories.sort_by(|a, b| b.cmp(a));

    calories.iter().take(3).sum()
}
