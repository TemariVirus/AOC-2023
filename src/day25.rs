use std::collections::{HashMap, HashSet, VecDeque};

const INPUT: &str = include_str!("day25.txt");

type Graph = HashMap<u16, HashSet<u16>>;

fn parse_input() -> Graph {
    let mut node_ids = HashMap::new();
    let mut nodes = HashMap::new();
    for line in INPUT.lines() {
        let (start, ends) = line.split_once(": ").unwrap();
        let ends: Vec<_> = ends.split(' ').collect();
        for &end in ends.iter() {
            let id = node_ids.len() as u16;
            node_ids.entry(end).or_insert(id);
        }
        let id = node_ids.len() as u16;
        node_ids.entry(start).or_insert(id);

        let start = node_ids[&start];
        let ends = ends.iter().map(|&e| node_ids[&e]);
        for end in ends.clone() {
            nodes.entry(end).or_insert_with(HashSet::new).insert(start);
        }
        nodes.entry(start).or_insert_with(HashSet::new).extend(ends);
    }

    nodes
}

#[allow(dead_code)]
pub fn part1() -> usize {
    let mut components = parse_input();

    for i in 1..components.len() {
        let paths = (0..3)
            .map(|_| {
                let path = pathfind(&components, 0, i as u16).unwrap();
                path.windows(2).for_each(|e| {
                    components.get_mut(&e[0]).unwrap().remove(&e[1]);
                    components.get_mut(&e[1]).unwrap().remove(&e[0]);
                });
                path
            })
            .collect::<Vec<_>>();

        match pathfind(&components, 0, i as u16) {
            // There is still a path, the components are in the same group
            Some(_) => (),
            // All 3 connecting edges have been removed, the components are in different groups
            None => {
                let size1 = connected_count(&components);
                let size2 = components.len() - size1;
                return size1 * size2;
            }
        }

        paths.into_iter().for_each(|path| {
            path.windows(2).for_each(|e| {
                components.get_mut(&e[0]).unwrap().insert(e[1]);
                components.get_mut(&e[1]).unwrap().insert(e[0]);
            });
        })
    }

    0
}

fn connected_count(nodes: &Graph) -> usize {
    let mut visited = HashSet::new();
    let mut queue = Vec::new();
    queue.push(*nodes.keys().next().unwrap());
    while let Some(node) = queue.pop() {
        if !visited.insert(node) {
            continue;
        }
        for &neighbor in nodes[&node].iter() {
            queue.push(neighbor);
        }
    }
    visited.len()
}

fn pathfind(nodes: &Graph, start: u16, end: u16) -> Option<Vec<u16>> {
    // Dijkstra's algorithm
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut parents = HashMap::new();
    queue.push_back(start);
    while let Some(node) = queue.pop_front() {
        if !visited.insert(node) {
            continue;
        }
        if node == end {
            break;
        }
        for &neighbor in nodes[&node].iter() {
            if !visited.contains(&neighbor) {
                parents.insert(neighbor, node);
                queue.push_back(neighbor);
            }
        }
    }

    let mut path = Vec::new();
    let mut node = end;
    while node != start {
        path.push(node);
        if parents.contains_key(&node) {
            node = parents[&node];
        } else {
            return None;
        }
    }
    path.push(start);
    path.reverse();
    Some(path)
}
