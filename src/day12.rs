use std::{collections::HashMap, iter::repeat};

const INPUT: &str = include_str!("day12.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

fn parse_input() -> Vec<(Vec<Spring>, Vec<u8>)> {
    INPUT
        .lines()
        .map(|line| {
            let (springs, runs) = line.split_once(' ').unwrap();
            let springs = springs
                .chars()
                .map(|c| match c {
                    '.' => Spring::Operational,
                    '#' => Spring::Damaged,
                    '?' => Spring::Unknown,
                    _ => unreachable!(),
                })
                .collect();
            let runs = runs.split(',').map(|run| run.parse().unwrap()).collect();
            (springs, runs)
        })
        .collect()
}

fn count_possibilities(
    springs: &[Spring],
    runs: &[u8],
    mut prev_damaged: bool,
    seen: &mut HashMap<u16, u64>,
) -> u64 {
    let hash = (springs.len() | runs.len() << 8).try_into().unwrap();
    let hash = if prev_damaged { hash | 1 << 15 } else { hash };
    if let Some(&count) = seen.get(&hash) {
        return count;
    }

    let mut runs_idx = 0;
    let mut run = 0;
    if prev_damaged {
        if runs_idx >= runs.len() {
            return 0;
        }
        run = runs[runs_idx] - 1;
        runs_idx += 1;
    }

    for i in 0..springs.len() {
        match springs[i] {
            Spring::Operational => {
                // Expected a damaged spring but got an operational one
                if run > 0 {
                    return 0;
                }
                prev_damaged = false;
            }
            Spring::Damaged => {
                if run == 0 {
                    // Ran out of damaged runs when there are still damaged springs
                    if runs_idx >= runs.len() {
                        return 0;
                    }
                    // Reached end of damaged run but there is another damaged spring
                    if prev_damaged {
                        return 0;
                    }
                    // Start the next damaged run
                    run = runs[runs_idx];
                    runs_idx += 1;
                }
                prev_damaged = true;
                run -= 1;
            }
            Spring::Unknown => {
                // Run is ongoing, this one must be damaged
                if run > 0 {
                    prev_damaged = true;
                    run -= 1;
                    continue;
                }
                // Run just ended, this one must be operational
                if prev_damaged {
                    prev_damaged = false;
                    continue;
                }

                // Undecided, try both possibilities
                let result = count_possibilities(&springs[i + 1..], &runs[runs_idx..], false, seen)
                    + count_possibilities(&springs[i + 1..], &runs[runs_idx..], true, seen);
                seen.insert(hash, result);
                return result;
            }
        }
    }

    let result = if run > 0 || runs_idx < runs.len() {
        // There are still damaged runs left, but no more damaged springs
        0
    } else {
        // No unknowns, only one possibility
        1
    };
    seen.insert(hash, result);
    result
}

#[allow(dead_code)]
pub fn part1() -> u64 {
    parse_input()
        .into_iter()
        .map(|(springs, runs)| count_possibilities(&springs, &runs, false, &mut HashMap::new()))
        .sum()
}

#[allow(dead_code)]
pub fn part2() -> u64 {
    parse_input()
        .into_iter()
        .map(|(springs, runs)| {
            (
                repeat([Spring::Unknown].into_iter().chain(springs))
                    .take(5)
                    .flatten()
                    .skip(1)
                    .collect::<Vec<_>>(),
                repeat(runs).take(5).flatten().collect::<Vec<_>>(),
            )
        })
        .map(|(springs, runs)| count_possibilities(&springs, &runs, false, &mut HashMap::new()))
        .sum()
}
