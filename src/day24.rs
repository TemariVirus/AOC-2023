use std::{collections::HashMap, num::ParseFloatError, ops::Index, str::FromStr};

const INPUT: &str = include_str!("day24.txt");

#[derive(Debug, Clone, Copy, PartialEq)]
struct Coord3 {
    x: f64,
    y: f64,
    z: f64,
}

impl FromStr for Coord3 {
    type Err = ParseFloatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, rest) = s.split_once(", ").unwrap();
        let (y, z) = rest.split_once(", ").unwrap();
        Ok(Coord3 {
            x: x.trim().parse()?,
            y: y.trim().parse()?,
            z: z.trim().parse()?,
        })
    }
}

impl Index<usize> for Coord3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out of bounds"),
        }
    }
}

fn parse_input() -> Vec<(Coord3, Coord3)> {
    INPUT
        .lines()
        .map(|line| {
            let (pos, vel) = line.split_once(" @ ").unwrap();
            (pos.parse().unwrap(), vel.parse().unwrap())
        })
        .collect()
}

#[allow(dead_code)]
pub fn part1() -> u16 {
    const MIN_COORD: f64 = 200000000000000.0;
    const MAX_COORD: f64 = 400000000000000.0;

    let hailstones = parse_input();
    hailstones
        .iter()
        .enumerate()
        .map(|(i, h1)| {
            hailstones[i + 1..]
                .iter()
                .filter(|h2| {
                    let p = ray_intersect2(h1.0, h1.1, h2.0, h2.1);
                    p.0 >= MIN_COORD && p.0 <= MAX_COORD && p.1 >= MIN_COORD && p.1 <= MAX_COORD
                })
                .count() as u16
        })
        .sum()
}

fn ray_intersect2(pos1: Coord3, vel1: Coord3, pos2: Coord3, vel2: Coord3) -> (f64, f64) {
    // Ray intersection algorithm
    // https://stackoverflow.com/a/2932601
    let det = vel2.x * vel1.y - vel2.y * vel1.x;
    if det == 0.0 {
        // Rays are parallel
        return (f64::NAN, f64::NAN);
    }

    let dx = pos2.x - pos1.x;
    let dy = pos2.y - pos1.y;
    let u = (dy * vel2.x - dx * vel2.y) / det;
    let v = (dy * vel1.x - dx * vel1.y) / det;
    if u < 0.0 || v < 0.0 {
        // Rays don't intersect
        return (f64::NAN, f64::NAN);
    }

    let x = pos1.x + u * vel1.x;
    let y = pos1.y + u * vel1.y;
    (x, y)
}

#[allow(dead_code)]
pub fn part2() -> i64 {
    let input = parse_input();

    // Problem reduced to linear algebra by someone much smarter than me
    // https://www.reddit.com/r/adventofcode/comments/18pnycy/comment/kepu26z/
    let mut results = Vec::new();
    for hailstones in input.chunks_exact(3) {
        let coeffs = vec![
            vec![
                hailstones[1].1[1] - hailstones[0].1[1],
                hailstones[0].1[0] - hailstones[1].1[0],
                0.0,
                hailstones[0].0[1] - hailstones[1].0[1],
                hailstones[1].0[0] - hailstones[0].0[0],
                0.0,
            ],
            vec![
                0.0,
                hailstones[1].1[2] - hailstones[0].1[2],
                hailstones[0].1[1] - hailstones[1].1[1],
                0.0,
                hailstones[0].0[2] - hailstones[1].0[2],
                hailstones[1].0[1] - hailstones[0].0[1],
            ],
            vec![
                hailstones[0].1[2] - hailstones[1].1[2],
                0.0,
                hailstones[1].1[0] - hailstones[0].1[0],
                hailstones[1].0[2] - hailstones[0].0[2],
                0.0,
                hailstones[0].0[0] - hailstones[1].0[0],
            ],
            vec![
                hailstones[2].1[1] - hailstones[0].1[1],
                hailstones[0].1[0] - hailstones[2].1[0],
                0.0,
                hailstones[0].0[1] - hailstones[2].0[1],
                hailstones[2].0[0] - hailstones[0].0[0],
                0.0,
            ],
            vec![
                0.0,
                hailstones[2].1[2] - hailstones[0].1[2],
                hailstones[0].1[1] - hailstones[2].1[1],
                0.0,
                hailstones[0].0[2] - hailstones[2].0[2],
                hailstones[2].0[1] - hailstones[0].0[1],
            ],
            vec![
                hailstones[0].1[2] - hailstones[2].1[2],
                0.0,
                hailstones[2].1[0] - hailstones[0].1[0],
                hailstones[2].0[2] - hailstones[0].0[2],
                0.0,
                hailstones[0].0[0] - hailstones[2].0[0],
            ],
        ];
        let consts = (0..6)
            .map(|i| {
                let dim1 = i % 3;
                let dim2 = (i + 1) % 3;
                let stone1 = 0;
                let stone2 = i / 3 + 1;
                hailstones[stone1].0[dim2] * hailstones[stone1].1[dim1]
                    - hailstones[stone1].0[dim1] * hailstones[stone1].1[dim2]
                    + hailstones[stone2].0[dim1] * hailstones[stone2].1[dim2]
                    - hailstones[stone2].0[dim2] * hailstones[stone2].1[dim1]
            })
            .collect();
        // Parallel hailstones will have no solutions
        if let Some(x) = solve(coeffs, consts) {
            results.push((x[0] + x[1] + x[2]).round() as i64);
        }
    }

    // Get the most common result to compenstate for rounding errors
    let counts = results.iter().fold(HashMap::new(), |mut map, x| {
        *map.entry(x).or_insert(0) += 1;
        map
    });
    *counts
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .unwrap()
        .0
}

// Solves the linear equation Ax = b with Gaussian elimination
fn solve(mut coeffs: Vec<Vec<f64>>, mut consts: Vec<f64>) -> Option<Vec<f64>> {
    // Forward elimination
    for i in 0..coeffs.len() - 1 {
        let swap = match coeffs[i..].iter().position(|row| row[i] != 0.into()) {
            Some(swap) => swap + i,
            None => return None,
        };
        coeffs.swap(i, swap);
        consts.swap(i, swap);

        for j in i + 1..coeffs.len() {
            let factor = coeffs[j][i] / coeffs[i][i];
            for k in i..coeffs[j].len() {
                coeffs[j][k] -= factor * coeffs[i][k];
                // Rectify rounding error
                if coeffs[j][k] < 0.01 && coeffs[j][k] > -0.01 {
                    coeffs[j][k] = 0.0;
                }
            }
            consts[j] -= factor * consts[i];
        }
    }

    // Back substitution
    for i in (0..coeffs.len()).rev() {
        for j in i + 1..coeffs.len() {
            consts[i] -= coeffs[i][j] * consts[j];
        }
        consts[i] /= coeffs[i][i];
    }

    Some(consts)
}
