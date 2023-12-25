use std::collections::HashMap;

const INPUT: &str = include_str!("day14.txt");

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Rock {
    Round,
    Cube,
    Empty,
}

fn parse_input() -> Vec<Vec<Rock>> {
    INPUT
        .lines()
        .map(|line| {
            line.bytes()
                .map(|c| match c {
                    b'O' => Rock::Round,
                    b'#' => Rock::Cube,
                    b'.' => Rock::Empty,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect()
}

#[allow(dead_code)]
pub fn part1() -> usize {
    let mut rocks = parse_input();
    tilt_north(&mut rocks);
    total_load(&rocks)
}

fn tilt_north(rocks: &mut Vec<Vec<Rock>>) {
    for i in 1..rocks.len() {
        for x in 0..rocks[i].len() {
            if rocks[i][x] != Rock::Round {
                continue;
            }

            let mut y = i;
            while y > 0 && rocks[y - 1][x] == Rock::Empty {
                y -= 1;
            }
            rocks[i][x] = Rock::Empty;
            rocks[y][x] = Rock::Round;
        }
    }
}

fn total_load(rocks: &Vec<Vec<Rock>>) -> usize {
    rocks
        .iter()
        .enumerate()
        .map(|(i, row)| (rocks.len() - i) * row.iter().filter(|&&rock| rock == Rock::Round).count())
        .sum()
}

#[allow(dead_code)]
pub fn part2() -> usize {
    const CYCLES: u32 = 1_000_000_000;

    let mut rocks = parse_input();
    let mut seen = HashMap::new();

    for i in 0..CYCLES {
        if let Some(cycle_start) = seen.get(&rocks) {
            let cycle_len = i - cycle_start;
            let remaining = (CYCLES - i) % cycle_len;

            let rocks = seen
                .iter()
                .find(|(_, &v)| v == cycle_start + remaining)
                .unwrap()
                .0;
            return total_load(rocks);
        }

        seen.insert(rocks.clone(), i);
        spin_cycle(&mut rocks);
    }

    total_load(&rocks)
}

fn spin_cycle(rocks: &mut Vec<Vec<Rock>>) {
    tilt_north(rocks);

    // West
    for i in 1..rocks[0].len() {
        for row in rocks.iter_mut() {
            if row[i] != Rock::Round {
                continue;
            }

            let mut x = i;
            while x > 0 && row[x - 1] == Rock::Empty {
                x -= 1;
            }
            row[i] = Rock::Empty;
            row[x] = Rock::Round;
        }
    }

    // South
    for i in (0..rocks.len() - 1).rev() {
        for x in 0..rocks[i].len() {
            if rocks[i][x] != Rock::Round {
                continue;
            }

            let mut y = i;
            while y < rocks.len() - 1 && rocks[y + 1][x] == Rock::Empty {
                y += 1;
            }
            rocks[i][x] = Rock::Empty;
            rocks[y][x] = Rock::Round;
        }
    }

    // East
    for i in (0..rocks[0].len() - 1).rev() {
        for y in 0..rocks.len() {
            if rocks[y][i] != Rock::Round {
                continue;
            }

            let mut x = i;
            while x < rocks[0].len() - 1 && rocks[y][x + 1] == Rock::Empty {
                x += 1;
            }
            rocks[y][i] = Rock::Empty;
            rocks[y][x] = Rock::Round;
        }
    }
}
