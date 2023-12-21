use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

const INPUT: &str = include_str!("day21.txt");

fn parse_input() -> (Vec<Vec<bool>>, (u8, u8)) {
    let mut start = (0, 0);
    let map = INPUT
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '#' => true,
                    '.' => false,
                    'S' => {
                        start = (x as u8, y as u8);
                        false
                    }
                    _ => panic!("Invalid input"),
                })
                .collect()
        })
        .collect();
    (map, start)
}

#[allow(dead_code)]
pub fn part1() -> u16 {
    const MAX_STEPS: u16 = 64;

    let (map, start) = parse_input();
    let dists = dijkstra(&map, start, MAX_STEPS);
    dists
        .iter()
        .flatten()
        .filter(|&&d| d % 2 == MAX_STEPS % 2)
        .count() as u16
}

fn dijkstra(map: &[Vec<bool>], start: (u8, u8), max_dist: u16) -> Vec<Vec<u16>> {
    let mut dists = vec![vec![u16::MAX; map[0].len()]; map.len()];
    dists[start.1 as usize][start.0 as usize] = 0;

    let mut queue = BinaryHeap::new();
    queue.push((Reverse(0), start.0, start.1));
    while let Some((Reverse(d), x, y)) = queue.pop() {
        for (dx, dy) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let new_x = x as i16 + dx;
            let new_y = y as i16 + dy;
            if new_x < 0
                || new_y < 0
                || new_x >= map[0].len() as i16
                || new_y >= map.len() as i16
                || map[new_y as usize][new_x as usize]
            {
                continue;
            }
            let new_x = new_x as u8;
            let new_y = new_y as u8;

            let new_d = d + 1;
            // We can skip if we're at the max distance or if this route is longer
            if new_d <= max_dist && new_d < dists[new_y as usize][new_x as usize] {
                dists[new_y as usize][new_x as usize] = new_d;
                queue.push((Reverse(new_d), new_x, new_y));
            }
        }
    }

    dists
}

#[allow(dead_code)]
pub fn part2() -> u64 {
    const MAX_STEPS: i64 = 26501365; // 26501365 = 202300 * 131 (input width) + 65 (half of 131)

    let (map, start) = parse_input();
    let mut points = (0..3).map(|i| {
        let steps = 131 * i + 65;
        let count = count_reachable(&map, start, steps);
        (i as i64, count as i64)
    });
    let (a, b, c) = fit_quadratic([
        points.next().unwrap(),
        points.next().unwrap(),
        points.next().unwrap(),
    ]);

    let x = (MAX_STEPS - 65) / 131;
    (a * x * x + b * x + c) as u64
}

fn count_reachable(map: &[Vec<bool>], start: (u8, u8), max_dist: u32) -> u64 {
    let start = (start.0 as i32, start.1 as i32);
    let mut count = (max_dist as u64 + 1) % 2;
    let mut visited = HashSet::new();
    let mut queue = BinaryHeap::new();

    visited.insert(start);
    queue.push((Reverse(0), start));

    while let Some((Reverse(d), current)) = queue.pop() {
        for (dx, dy) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let next = (current.0 + dx, current.1 + dy);
            if map[next.1.rem_euclid(map.len() as i32) as usize]
                [next.0.rem_euclid(map[0].len() as i32) as usize]
            {
                continue;
            }
            if !visited.insert(next) {
                continue;
            }
            if (d + 1) % 2 == max_dist % 2 {
                count += 1;
            }
            if d + 1 >= max_dist {
                continue;
            }

            queue.push((Reverse(d + 1), next));
        }
    }

    count
}

fn fit_quadratic(points: [(i64, i64); 3]) -> (i64, i64, i64) {
    let (x1, y1) = points[0];
    let (x2, y2) = points[1];
    let (x3, y3) = points[2];

    // Lagrange interpolation
    let a =
        (y1 * (x2 - x3) + y2 * (x3 - x1) + y3 * (x1 - x2)) / ((x1 - x2) * (x1 - x3) * (x2 - x3));
    let b = (y1 - y2) / (x1 - x2) - a * (x1 + x2);
    let c = y1 - a * x1 * x1 - b * x1;

    (a, b, c)
}
