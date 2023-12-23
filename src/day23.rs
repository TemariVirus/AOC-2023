use std::collections::{BinaryHeap, HashMap, HashSet};

const INPUT: &str = include_str!("day23.txt");

type Edge = (u16, (u8, u8), (u8, u8));

#[derive(Debug, Clone, Copy)]
struct BitSet {
    data: u64,
}

impl BitSet {
    fn new() -> Self {
        Self { data: 0 }
    }

    fn insert(&mut self, i: u8) {
        self.data |= 1 << i;
    }

    fn contains(&self, i: u8) -> bool {
        (self.data >> i) & 1 == 1
    }
}

fn parse_input() -> Vec<Vec<bool>> {
    // Slopes only point down and right, so we can ignore them
    INPUT
        .lines()
        .map(|line| line.chars().map(|c| c == '#').collect())
        .collect()
}

// Vertices of edges are guaranteed to be ordered such that the graph is a DAG
fn compress_graph(map: &[Vec<bool>], start: (u8, u8), end: (u8, u8)) -> Vec<Edge> {
    let mut blocked = HashSet::new();
    let mut edges = Vec::new();

    let mut open = vec![start];
    while let Some((mut x, mut y)) = open.pop() {
        if (x, y) == end {
            continue;
        }

        let mut neighbours: Vec<_> = [(x + 1, y), (x, y + 1)]
            .into_iter()
            .filter(|&(next_x, next_y)| {
                !map[next_y as usize][next_x as usize] && !blocked.contains(&(next_x, next_y))
            })
            .collect();
        if neighbours.is_empty() {
            continue;
        }

        let from = (x, y);
        let mut prev = (x, y);
        (x, y) = neighbours[0];
        let mut len = 0;

        // Block first node
        blocked.insert((x, y));

        loop {
            neighbours = [(x + 1, y), (x, y + 1), (x - 1, y), (x, y - 1)]
                .into_iter()
                .filter(|&(next_x, next_y)| {
                    prev != (next_x, next_y)
                        && (next_y as usize) < map.len()
                        && !map[next_y as usize][next_x as usize]
                })
                .collect();

            len += 1;
            if neighbours.len() != 1 {
                break;
            }

            prev = (x, y);
            (x, y) = neighbours[0];
        }

        // Block last node
        if len > 1 {
            blocked.insert(prev);
        }
        edges.push((len, from, (x, y)));

        open.push((x, y));
        open.push(from);
    }

    edges
}

#[allow(dead_code)]
pub fn part1() -> u16 {
    let map = parse_input();
    let start = (1, 0);
    let end = (map[0].len() as u8 - 2, map.len() as u8 - 1);

    let edges = compress_graph(&map, start, end);
    longest_path_dag(&edges, start, end)
}

fn longest_path_dag(edges: &[Edge], start: (u8, u8), end: (u8, u8)) -> u16 {
    // A* with negative weights
    let h = |(x, y): (u8, u8)| (end.0 - x) as u16 + (end.1 - y) as u16;

    let mut g_scores = HashMap::new();
    g_scores.insert(start, 0);

    let mut open = BinaryHeap::new();
    open.push((h(start), start));

    while let Some((_, (x, y))) = open.pop() {
        if (x, y) == end {
            return 0u16.wrapping_sub(g_scores[&(x, y)]);
        }

        let g_score = g_scores[&(x, y)];
        for &(weight, from, to) in edges {
            if from != (x, y) {
                continue;
            }

            let g_score = g_score.wrapping_sub(weight);
            if g_score < *g_scores.get(&to).unwrap_or(&u16::MAX) {
                g_scores.insert(to, g_score);
                let f_score = g_score.wrapping_add(h(to));
                open.push((f_score, to));
            }
        }
    }

    panic!("No path found");
}

#[allow(dead_code)]
pub fn part2() -> u16 {
    let map = parse_input();
    let start = (1, 0);
    let end = (map[0].len() as u8 - 2, map.len() as u8 - 1);

    let edges = compress_graph(&map, start, end);
    longest_path(&edges, start, end)
}

fn longest_path(edges: &[Edge], start: (u8, u8), end: (u8, u8)) -> u16 {
    let mut vertices = HashMap::new();
    for &(_, from, to) in edges {
        let id = vertices.len() as u8;
        vertices.entry(from).or_insert_with(|| id);
        let id = vertices.len() as u8;
        vertices.entry(to).or_insert_with(|| id);
    }

    let mut edges: Vec<_> = vertices
        .iter()
        .map(|(&k, &v)| {
            (
                v,
                edges
                    .iter()
                    .filter(|&&(_, from, to)| from == k || to == k)
                    .map(|&(weight, from, to)| {
                        (weight, vertices[&if from == k { to } else { from }])
                    })
                    .collect::<Vec<_>>(),
            )
        })
        .collect();
    edges.sort_unstable_by_key(|&(k, _)| k);
    let edges: Vec<_> = edges.into_iter().map(|(_, v)| v).collect();

    let start = vertices[&start];
    let end = vertices[&end];

    let mut max = 0;
    let mut stack = Vec::new();
    stack.push((start, 0, BitSet::new()));
    while let Some((start, acc, mut visited)) = stack.pop() {
        if start == end {
            if acc > max {
                max = acc;
            }
            continue;
        }

        visited.insert(start);
        for &(weight, next) in edges[start as usize].iter() {
            if !visited.contains(next) {
                stack.push((next, acc + weight, visited));
            }
        }
    }

    max
}
