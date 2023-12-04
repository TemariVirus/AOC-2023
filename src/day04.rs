use std::{collections::HashSet, iter::repeat};

const INPUT: &str = include_str!("day04.txt");

fn parse_input() -> Vec<u8> {
    INPUT
        .lines()
        .map(|line| {
            let (_, line) = line.split_once(": ").unwrap();
            let (wins, nums) = line.split_once(" | ").unwrap();
            let numbers: HashSet<u8> =
                HashSet::from_iter(nums.split_whitespace().map(|s| s.parse().unwrap()));

            wins.split_whitespace()
                .map(|s| s.parse().unwrap())
                .filter(|n| numbers.contains(n))
                .count()
                .try_into()
                .unwrap()
        })
        .collect()
}

#[allow(dead_code)]
pub fn part1() -> u32 {
    parse_input().iter().map(|&m| 2u32.pow(m.into()) / 2).sum()
}

#[allow(dead_code)]
pub fn part2() -> u32 {
    let matches = parse_input();
    let mut counts = repeat(1u32).take(matches.len()).collect::<Vec<_>>();
    for i in 0..counts.len() - 1 {
        for j in i + 1..i + 1 + matches[i] as usize {
            counts[j] += counts[i];
        }
    }
    counts.iter().sum()
}
