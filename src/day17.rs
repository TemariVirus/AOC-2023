use std::collections::BinaryHeap;

const INPUT: &str = include_str!("day17.txt");

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Right = 0,
    Left,
    Down,
    Up,
    None,
}

#[derive(Debug, Clone, Copy)]
struct Node {
    x: u8,
    y: u8,
    d: Direction,
    g_score: u16,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.g_score == other.g_score
    }
}

impl Eq for Node {}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Reverse so that binary heap is a min heap
        self.g_score.cmp(&other.g_score).reverse()
    }
}

struct Array3D<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
    depth: usize,
}

impl<T> Array3D<T>
where
    T: Copy,
{
    fn new(width: usize, height: usize, depth: usize, default: T) -> Self {
        Self {
            data: vec![default; width * height * depth],
            width,
            height,
            depth,
        }
    }

    fn get(&self, x: u8, y: u8, z: u8) -> T {
        let (x, y, z) = (x as usize, y as usize, z as usize);

        debug_assert!(x < self.width && y < self.height && z < self.depth);
        self.data[x + y * self.width + z * self.width * self.height]
    }

    fn set(&mut self, x: u8, y: u8, z: u8, value: T) {
        let (x, y, z) = (x as usize, y as usize, z as usize);

        debug_assert!(x < self.width && y < self.height && z < self.depth);
        self.data[x + y * self.width + z * self.width * self.height] = value;
    }
}

fn parse_input() -> Vec<Vec<u8>> {
    INPUT
        .lines()
        .map(|line| {
            line.chars()
                .map(|b| b.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect()
}

fn lowest_loss(blocks: &[&[u8]], min_straight: u8, max_straight: u8) -> Option<u16> {
    // Dijkstra's algorithm
    let start_x = 0;
    let start_y = 0;
    let target_x = (blocks[0].len() - 1) as u8;
    let target_y = (blocks.len() - 1) as u8;

    let mut g_scores = Array3D::new(blocks[0].len(), blocks.len(), 5, u16::MAX);
    g_scores.set(start_x, start_y, Direction::None as u8, 0);

    let mut open = BinaryHeap::new();
    open.push(Node {
        x: start_x,
        y: start_y,
        d: Direction::None,
        g_score: g_scores.get(start_x, start_y, Direction::None as u8),
    });

    while let Some(Node { x, y, d, g_score }) = open.pop() {
        if (x, y) == (target_x, target_y) {
            return Some(g_score);
        }

        let directions = match d {
            Direction::Right | Direction::Left => [Direction::Down, Direction::Up],
            Direction::Down | Direction::Up => [Direction::Right, Direction::Left],
            Direction::None => [Direction::Right, Direction::Down],
        };
        for next_d in directions {
            let mut tentative_g_score = g_score;
            for step in 1..=max_straight {
                let (next_x, next_y) = match next_d {
                    Direction::Right => (x + step, y),
                    Direction::Left => (x.wrapping_sub(step), y),
                    Direction::Down => (x, y + step),
                    Direction::Up => (x, y.wrapping_sub(step)),
                    Direction::None => unreachable!(),
                };

                // Stay in bounds
                if next_x as usize >= blocks[0].len() || next_y as usize >= blocks.len() {
                    break;
                }

                tentative_g_score = tentative_g_score
                    .saturating_add(blocks[next_y as usize][next_x as usize] as u16);

                // Too early to turn
                if step < min_straight {
                    continue;
                }

                if tentative_g_score < g_scores.get(next_x, next_y, next_d as u8) {
                    // Found better path
                    g_scores.set(next_x, next_y, next_d as u8, tentative_g_score);

                    open.push(Node {
                        x: next_x,
                        y: next_y,
                        d: next_d,
                        g_score: tentative_g_score,
                    });
                }
            }
        }
    }

    None
}

#[allow(dead_code)]
pub fn part1() -> u16 {
    let blocks = parse_input();
    let blocks = blocks.iter().map(|row| row.as_slice()).collect::<Vec<_>>();
    lowest_loss(&blocks, 1, 3).unwrap()
}

#[allow(dead_code)]
pub fn part2() -> u16 {
    let blocks = parse_input();
    let blocks = blocks.iter().map(|row| row.as_slice()).collect::<Vec<_>>();
    lowest_loss(&blocks, 4, 10).unwrap()
}
