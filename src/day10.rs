use std::iter::repeat;

const INPUT: &str = include_str!("day10.txt");

#[allow(dead_code)]
pub fn part1() -> u32 {
    let grid = INPUT.lines().map(|s| s.as_bytes()).collect::<Vec<_>>();

    let (start_x, start_y) = find_start(&grid);
    let (mut x, mut y) = next_pipe(&grid, start_x, start_y, start_x, start_y);
    let mut steps = 1;
    let mut prev_x = start_x;
    let mut prev_y = start_y;

    while start_x != x || start_y != y {
        let (next_x, next_y) = next_pipe(&grid, x, y, prev_x, prev_y);
        prev_x = x;
        prev_y = y;
        x = next_x;
        y = next_y;
        steps += 1;
    }

    steps / 2
}

fn find_start(grid: &[&[u8]]) -> (usize, usize) {
    for (i, row) in grid.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == b'S' {
                return (j, i);
            }
        }
    }
    unreachable!()
}

fn next_pipe(grid: &[&[u8]], x: usize, y: usize, prev_x: usize, prev_y: usize) -> (usize, usize) {
    match grid[y][x] {
        b'|' => {
            if y > prev_y {
                (x, y + 1)
            } else {
                (x, y - 1)
            }
        }
        b'-' => {
            if x > prev_x {
                (x + 1, y)
            } else {
                (x - 1, y)
            }
        }
        b'L' => {
            if y > prev_y {
                (x + 1, y)
            } else {
                (x, y - 1)
            }
        }
        b'J' => {
            if y > prev_y {
                (x - 1, y)
            } else {
                (x, y - 1)
            }
        }
        b'7' => {
            if y < prev_y {
                (x - 1, y)
            } else {
                (x, y + 1)
            }
        }
        b'F' => {
            if y < prev_y {
                (x + 1, y)
            } else {
                (x, y + 1)
            }
        }
        b'S' => {
            // Left
            if x > 0 {
                let pipe = grid[y][x - 1];
                if pipe == b'-' || pipe == b'L' || pipe == b'F' {
                    return (x - 1, y);
                }
            }
            // Right
            if x < grid[y].len() - 1 {
                let pipe = grid[y][x + 1];
                if pipe == b'-' || pipe == b'J' || pipe == b'7' {
                    return (x + 1, y);
                }
            }
            // Up
            if y > 0 {
                let pipe = grid[y - 1][x];
                if pipe == b'|' || pipe == b'7' || pipe == b'F' {
                    return (x, y - 1);
                }
            }
            // Down
            if y < grid.len() - 1 {
                let pipe = grid[y + 1][x];
                if pipe == b'|' || pipe == b'J' || pipe == b'L' {
                    return (x, y + 1);
                }
            }
            unreachable!()
        }
        _ => unreachable!(),
    }
}

#[allow(dead_code)]
pub fn part2() -> u32 {
    let grid = INPUT.lines().map(|s| s.as_bytes()).collect::<Vec<_>>();
    let mut loop_grid = repeat(repeat(b'O').take(grid[0].len()).collect::<Vec<_>>())
        .take(grid.len())
        .collect::<Vec<_>>();

    let (start_x, start_y) = find_start(&grid);
    loop_grid[start_y][start_x] = start_type(&grid, start_x, start_y);
    let (mut x, mut y) = next_pipe(&grid, start_x, start_y, start_x, start_y);
    let mut prev_x = start_x;
    let mut prev_y = start_y;

    while start_x != x || start_y != y {
        loop_grid[y][x] = grid[y][x];
        let (next_x, next_y) = next_pipe(&grid, x, y, prev_x, prev_y);
        prev_x = x;
        prev_y = y;
        x = next_x;
        y = next_y;
    }

    let loop_grid = loop_grid.iter().map(|v| v.as_slice()).collect::<Vec<_>>();
    (0..loop_grid.len())
        .filter_map(|i| {
            if i % 2 == 0 {
                Some(count_inside(&loop_grid, i))
            } else {
                None
            }
        })
        .sum()
}

fn start_type(grid: &[&[u8]], x: usize, y: usize) -> u8 {
    // Assume start is not at the edge
    assert!(x > 0 && x < grid[y].len() - 1 && y > 0 && y < grid.len() - 1);

    let left = grid[y][x - 1];
    let right = grid[y][x + 1];
    let up = grid[y - 1][x];
    let down = grid[y + 1][x];
    if left == b'-' || left == b'L' || left == b'F' {
        if up == b'|' || up == b'7' || up == b'F' {
            b'J'
        } else if down == b'|' || down == b'J' || down == b'L' {
            b'7'
        } else {
            b'-'
        }
    } else if right == b'-' || right == b'J' || right == b'7' {
        if up == b'|' || up == b'7' || up == b'F' {
            b'L'
        } else if down == b'|' || down == b'J' || down == b'L' {
            b'F'
        } else {
            b'-'
        }
    } else {
        b'|'
    }
}

fn count_inside(grid: &[&[u8]], row: usize) -> u32 {
    if row >= grid.len() - 1 {
        return 0;
    }

    let top = grid[row];
    let bottom = grid[row + 1];
    let mut count = 0;
    let mut inside = false;
    for i in 1..top.len() {
        if top[i] == b'|' || top[i] == b'7' || top[i] == b'F' {
            inside = !inside;
            continue;
        }
        if inside {
            if top[i] == b'O' {
                count += 1;
            }
            if bottom[i] == b'O' {
                count += 1;
            }
        }
    }

    count
}
