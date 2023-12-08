use std::{collections::HashMap, iter::repeat};

const INPUT: &str = include_str!("day08.txt");

fn name_to_id(name: &[u8]) -> u16 {
    let mut id: u16 = 0;
    id += (name[0] - b'A') as u16;
    id *= 26;
    id += (name[1] - b'A') as u16;
    id *= 26;
    id += (name[2] - b'A') as u16;
    id
}

fn parse_input() -> (&'static str, HashMap<u16, (u16, u16)>) {
    let (instructions, graph) = INPUT.split_once("\n\n").unwrap();
    let graph = graph
        .lines()
        .map(|line| {
            let node = name_to_id(&line.as_bytes()[0..3]);
            let left = name_to_id(&line.as_bytes()[7..10]);
            let right = name_to_id(&line.as_bytes()[12..15]);
            (node, (left, right))
        })
        .collect();
    (instructions, graph)
}

fn next_node(instruction: char, current: u16, graph: &HashMap<u16, (u16, u16)>) -> u16 {
    let node = graph.get(&current).unwrap();
    match instruction {
        'L' => node.0,
        'R' => node.1,
        _ => unreachable!(),
    }
}

#[allow(dead_code)]
pub fn part1() -> u32 {
    const START: u16 = 0;
    const END: u16 = 26 * 26 * 26 - 1;

    let (instructions, graph) = parse_input();
    // Repeat infinitely
    let instructions = repeat(instructions.chars()).flatten();

    let mut steps = 0;
    let mut current = START;
    for instruct in instructions {
        if current == END {
            break;
        }
        current = next_node(instruct, current, &graph);
        steps += 1;
    }
    steps
}

#[allow(dead_code)]
pub fn part2() -> u64 {
    let (instructions, graph) = parse_input();
    // Repeat infinitely
    let instructions = repeat(instructions.chars()).flatten();

    graph
        .keys()
        .filter(|&k| ends_with_a(k))
        .map(|&node| {
            let mut current = node;
            let mut steps = 0;
            let instructions = instructions.clone();
            for instruct in instructions {
                if ends_with_z(&current) {
                    break;
                }
                current = next_node(instruct, current, &graph);
                steps += 1;
            }
            // The XXZ nodes seem to always go back to the original XXA node,
            // so loop_size = initial_steps_to_XXZ
            steps
        })
        .reduce(|acc, x| {
            // The closed loops also means that the answer is just the LCM of
            // the loop sizes
            let gcd = gcd(acc, x);
            acc / gcd * x
        })
        .unwrap()
}

fn ends_with_a(id: &u16) -> bool {
    id % 26 == 0
}

fn ends_with_z(id: &u16) -> bool {
    id % 26 == 25
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}
