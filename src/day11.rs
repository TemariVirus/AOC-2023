const INPUT: &str = include_str!("day11.txt");

fn parse_input(empty_size: u64) -> Vec<(u64, u64)> {
    let map = INPUT
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut x_dists = vec![0; map[0].len()];
    for x in 1..map[0].len() {
        let empty = (0..map.len()).all(|y| map[y][x] == '.');
        x_dists[x] = x_dists[x - 1] + if empty { empty_size } else { 1 };
    }

    let mut y_dists = vec![0; map.len()];
    for y in 1..map.len() {
        let empty = map[y].iter().all(|&c| c == '.');
        y_dists[y] = y_dists[y - 1] + if empty { empty_size } else { 1 };
    }

    map.iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, &c)| c == '#')
                .map(|(x, _)| (x_dists[x], y_dists[y]))
                .collect::<Vec<_>>()
        })
        .collect()
}

fn sum_pair_dists(galaxies: Vec<(u64, u64)>) -> u64 {
    galaxies
        .iter()
        .enumerate()
        .map(|(i, g1)| {
            galaxies[i + 1..]
                .iter()
                .map(|g2| g1.0.abs_diff(g2.0) + g1.1.abs_diff(g2.1))
                .sum::<u64>()
        })
        .sum()
}

#[allow(dead_code)]
pub fn part1() -> u64 {
    sum_pair_dists(parse_input(2))
}

#[allow(dead_code)]
pub fn part2() -> u64 {
    sum_pair_dists(parse_input(1_000_000))
}
