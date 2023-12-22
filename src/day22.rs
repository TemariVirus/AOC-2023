use std::{collections::HashSet, iter::repeat, str::FromStr};

const INPUT: &str = include_str!("day22.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Brick {
    x1: u16,
    x2: u16,
    y1: u16,
    y2: u16,
    z1: u16,
    z2: u16,
}

impl Brick {
    fn set_z1(&mut self, z: u16) {
        self.z2 = self.z2 - self.z1 + z;
        self.z1 = z;
    }

    fn intersects(&self, other: &Self) -> bool {
        self.x1 <= other.x2
            && self.x2 >= other.x1
            && self.y1 <= other.y2
            && self.y2 >= other.y1
            && self.z1 <= other.z2
            && self.z2 >= other.z1
    }
}

impl FromStr for Brick {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (p1, p2) = s.split_once('~').unwrap();
        let p1: Vec<u16> = p1.split(',').map(|x| x.parse().unwrap()).collect();
        let p2: Vec<u16> = p2.split(',').map(|x| x.parse().unwrap()).collect();
        Ok(Brick {
            x1: p1[0].min(p2[0]),
            x2: p1[0].max(p2[0]),
            y1: p1[1].min(p2[1]),
            y2: p1[1].max(p2[1]),
            z1: p1[2].min(p2[2]),
            z2: p1[2].max(p2[2]),
        })
    }
}

fn parse_input() -> Vec<Brick> {
    INPUT.lines().map(|line| line.parse().unwrap()).collect()
}

fn brick_fall(bricks: &mut [Brick]) {
    bricks.sort_unstable_by_key(|b| b.z1);
    bricks[0].set_z1(1);

    for i in 1..bricks.len() {
        // Find the highest brick that supports this one
        for j in (0..i).rev() {
            bricks[i].set_z1(bricks[j].z2);
            if bricks[i].intersects(&bricks[j]) {
                bricks[i].set_z1(bricks[j].z2 + 1);
                break;
            }
        }
        // No brick supports this one, so it falls to the ground
        if bricks[i].z1 == bricks[0].z2 {
            bricks[i].set_z1(1);
        }

        // Arrange bricks in order of increasing top height (z2)
        let idx = bricks[..i].partition_point(|b| b.z2 <= bricks[i].z2);
        bricks[idx..=i].rotate_right(1);
    }
}

#[allow(dead_code)]
pub fn part1() -> u16 {
    let mut bricks = parse_input();
    brick_fall(&mut bricks);
    count_safe(&mut bricks)
}

fn count_safe(bricks: &mut [Brick]) -> u16 {
    bricks.sort_unstable_by_key(|b| b.z2);
    let mut safe: Vec<_> = repeat(true).take(bricks.len()).collect();

    for (i, mut brick) in bricks.iter().copied().enumerate() {
        brick.z1 -= 1;
        brick.z2 -= 1;

        let mut under_idx = i;
        for (j, other) in bricks[..i].iter().enumerate().rev() {
            if brick.z1 > other.z2 {
                break;
            }
            if !brick.intersects(other) {
                continue;
            }

            // If there's a second brick under this one, it's safe to remove them
            if under_idx != i {
                under_idx = i;
                break;
            }
            under_idx = j;
        }

        // If there's only one brick under this one, it's not safe to remove it
        if under_idx != i {
            safe[under_idx] = false;
        }
    }

    safe.iter().filter(|&&x| x).count() as u16
}

#[allow(dead_code)]
pub fn part2() -> u32 {
    let mut bricks = parse_input();
    brick_fall(&mut bricks);
    count_fall(&bricks)
}

fn count_fall(bricks: &[Brick]) -> u32 {
    let mut indices: Vec<_> = (0..bricks.len()).collect();
    indices.sort_unstable_by_key(|&i| bricks[i].z2);

    let unders: Vec<_> = indices
        .iter()
        .map(|&i| {
            let mut brick = bricks[i];
            brick.z1 -= 1;
            brick.z2 -= 1;

            let mut under = HashSet::new();
            for (j, other) in bricks[..i].iter().enumerate().rev() {
                if brick.z1 > other.z2 {
                    break;
                }
                if brick.intersects(other) {
                    under.insert(j);
                }
            }
            under
        })
        .collect();

    let mut n_fell = 0;
    indices.sort_unstable_by_key(|&i| bricks[i].z1);
    for &i in &indices {
        let brick = bricks[i];
        let start = indices.partition_point(|&i| bricks[i].z1 <= brick.z2);
        if start == bricks.len() {
            continue;
        }

        let mut fell = HashSet::new();
        let mut max_z2 = brick.z2;
        fell.insert(i);
        for &j in &indices[start..] {
            let other = bricks[j];
            if other.z1 > max_z2 + 1 {
                break;
            }

            if unders[j].is_subset(&fell) {
                max_z2 = max_z2.max(other.z2);
                fell.insert(j);
            }
        }
        n_fell += fell.len() as u32 - 1;
    }
    n_fell
}
