use std::iter::repeat;

const INPUT: &str = include_str!("day03.txt");

#[derive(Debug, Clone, Copy)]
enum Cell {
    Empty,
    Number(Number),
    PartNum(Number),
    Symbol(u8),
}

#[derive(Debug, Clone, Copy)]
struct Number {
    id: u16,
    value: u16,
}

fn parse_input(input: &str) -> Vec<Vec<Cell>> {
    let mut id_counter = 0;
    let mut cells: Vec<Vec<_>> = input
        .lines()
        .map(|line| {
            let line = line.as_bytes(); // All characters in input are ASCII
            let mut row = Vec::with_capacity(line.len());

            let mut i = 0;
            while i < line.len() {
                let c = line[i];
                match c {
                    b'.' => {
                        row.push(Cell::Empty);
                        i += 1;
                    }
                    b'0'..=b'9' => {
                        let mut value = 0;
                        let start = i;
                        while i < line.len() && line[i].is_ascii_digit() {
                            value = value * 10 + (line[i] - b'0') as u16;
                            i += 1;
                        }
                        row.extend(
                            repeat(Cell::Number(Number {
                                id: id_counter,
                                value,
                            }))
                            .take(i - start),
                        );
                        id_counter += 1;
                    }
                    _ => {
                        row.push(Cell::Symbol(c));
                        i += 1;
                    }
                }
            }

            row
        })
        .collect();

    find_part_nums(&mut cells);
    cells
}

fn find_part_nums(schema: &mut [Vec<Cell>]) {
    let old = schema.to_owned();
    for (i, row) in schema.iter_mut().enumerate() {
        for (j, cell) in row.iter_mut().enumerate() {
            if let Cell::Number(number) = cell {
                let x_start = j.saturating_sub(1);
                let x_end = (j + 2).min(old[i].len());
                let y_start = i.saturating_sub(1);
                let y_end = (i + 2).min(old.len());

                'outer: for row in &old[y_start..y_end] {
                    for c in &row[x_start..x_end] {
                        if let Cell::Symbol(_) = c {
                            *cell = Cell::PartNum(*number);
                            break 'outer;
                        }
                    }
                }
            }
        }
    }
}

#[allow(dead_code)]
pub fn part1() -> u32 {
    let schema = parse_input(INPUT);

    let mut max_id = 0;
    schema
        .iter()
        .flatten()
        .filter_map(|num| {
            if let Cell::PartNum(number) = num {
                if number.id >= max_id {
                    max_id = number.id + 1;
                    Some(number.value as u32)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .sum()
}

#[allow(dead_code)]
pub fn part2() -> u32 {
    let schema = parse_input(INPUT);
    schema
        .iter()
        .flatten()
        .enumerate()
        .filter_map(|(i, num)| {
            if let Cell::Symbol(b'*') = num {
                gear_ratio(&schema, i)
            } else {
                None
            }
        })
        .sum()
}

fn gear_ratio(schema: &[Vec<Cell>], i: usize) -> Option<u32> {
    let x = i % schema[0].len();
    let y = i / schema[0].len();

    let x_start = x.saturating_sub(1);
    let x_end = (x + 2).min(schema[0].len());
    let y_start = y.saturating_sub(1);
    let y_end = (y + 2).min(schema.len());

    let mut max_id = 0;
    let mut first = None;
    let mut second = None;
    for row in &schema[y_start..y_end] {
        for c in &row[x_start..x_end] {
            if let Cell::PartNum(number) = c {
                if number.id < max_id {
                    continue;
                }

                max_id = number.id + 1;
                if first.is_none() {
                    first = Some(number.value);
                } else if second.is_none() {
                    second = Some(number.value);
                } else {
                    return None;
                }
            }
        }
    }

    if let (Some(first), Some(second)) = (first, second) {
        Some(first as u32 * second as u32)
    } else {
        None
    }
}
