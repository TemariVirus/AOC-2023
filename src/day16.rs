use std::collections::HashSet;

const INPUT: &str = include_str!("day16.txt");

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Tile {
    Empty,
    NEMirror,
    NWMirror,
    VerticalSplit,
    HorizontalSplit,
}

fn parse_input() -> Vec<Vec<Tile>> {
    INPUT
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Tile::Empty,
                    '/' => Tile::NEMirror,
                    '\\' => Tile::NWMirror,
                    '|' => Tile::VerticalSplit,
                    '-' => Tile::HorizontalSplit,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect()
}

fn energized_map(
    mut x: i8,
    mut y: i8,
    mut dx: i8,
    mut dy: i8,
    tiles: &[&[Tile]],
    seen: &mut HashSet<(i8, i8)>,
    seen_with_d: &mut HashSet<(i8, i8, i8, i8)>,
) -> usize {
    while x >= 0 && y >= 0 && x < tiles[0].len() as i8 && y < tiles.len() as i8 {
        if !seen.insert((x, y)) && !seen_with_d.insert((x, y, dx, dy)) {
            break;
        }

        match tiles[y as usize][x as usize] {
            Tile::Empty => (),
            Tile::NEMirror => {
                (dx, dy) = (-dy, -dx);
            }
            Tile::NWMirror => {
                (dx, dy) = (dy, dx);
            }
            Tile::VerticalSplit => {
                if dx != 0 {
                    energized_map(x, y - 1, 0, -1, tiles, seen, seen_with_d);
                    energized_map(x, y + 1, 0, 1, tiles, seen, seen_with_d);
                    break;
                }
            }
            Tile::HorizontalSplit => {
                if dy != 0 {
                    energized_map(x - 1, y, -1, 0, tiles, seen, seen_with_d);
                    energized_map(x + 1, y, 1, 0, tiles, seen, seen_with_d);
                    break;
                }
            }
        }

        x += dx;
        y += dy;
    }

    seen.len()
}

#[allow(dead_code)]
pub fn part1() -> usize {
    let tiles = parse_input();
    let tiles = tiles.iter().map(|row| row.as_slice()).collect::<Vec<_>>();
    energized_map(0, 0, 1, 0, &tiles, &mut HashSet::new(), &mut HashSet::new())
}

#[allow(dead_code)]
pub fn part2() -> usize {
    let tiles = parse_input();
    let tiles = tiles.iter().map(|row| row.as_slice()).collect::<Vec<_>>();

    let mut max_energised = 0;
    let mut seen = HashSet::new();
    let mut seen_with_d = HashSet::new();
    // Test all edges
    for x in 0..tiles[0].len() {
        max_energised = max_energised.max(energized_map(
            x as i8,
            0,
            0,
            1,
            &tiles,
            &mut seen,
            &mut seen_with_d,
        ));
        seen.clear();
        seen_with_d.clear();

        max_energised = max_energised.max(energized_map(
            x as i8,
            tiles.len() as i8 - 1,
            0,
            -1,
            &tiles,
            &mut seen,
            &mut seen_with_d,
        ));
        seen.clear();
        seen_with_d.clear();
    }
    for y in 0..tiles.len() {
        max_energised = max_energised.max(energized_map(
            0,
            y as i8,
            1,
            0,
            &tiles,
            &mut seen,
            &mut seen_with_d,
        ));
        seen.clear();
        seen_with_d.clear();

        max_energised = max_energised.max(energized_map(
            tiles[0].len() as i8 - 1,
            y as i8,
            -1,
            0,
            &tiles,
            &mut seen,
            &mut seen_with_d,
        ));
        seen.clear();
        seen_with_d.clear();
    }

    max_energised
}
