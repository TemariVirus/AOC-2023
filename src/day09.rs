const INPUT: &str = include_str!("day09.txt");

fn parse_input() -> Vec<Vec<i32>> {
    INPUT
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect()
        })
        .collect()
}

#[allow(dead_code)]
pub fn part1() -> i32 {
    parse_input().iter().map(|row| next_reading(row)).sum()
}

fn next_reading(history: &[i32]) -> i32 {
    if history.iter().all(|&x| x == history[0]) {
        return history[0];
    }

    let diffs = history
        .windows(2)
        .map(|window| window[1] - window[0])
        .collect::<Vec<_>>();
    history[history.len() - 1] + next_reading(&diffs)
}

#[allow(dead_code)]
pub fn part2() -> i32 {
    parse_input().iter().map(|row| prev_reading(row)).sum()
}

fn prev_reading(history: &[i32]) -> i32 {
    if history.iter().all(|&x| x == history[0]) {
        return history[0];
    }

    let diffs = history
        .windows(2)
        .map(|window| window[1] - window[0])
        .collect::<Vec<_>>();
    history[0] - prev_reading(&diffs)
}
