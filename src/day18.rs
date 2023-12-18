const INPUT: &str = include_str!("day18.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

fn get_outline(plan: &[(Direction, i64)]) -> Vec<(i64, i64)> {
    let mut outline = vec![];
    let (mut x, mut y) = (0, 0);
    for &(d, steps) in plan {
        outline.push((x, y));
        match d {
            Direction::Right => x += steps,
            Direction::Down => y += steps,
            Direction::Left => x -= steps,
            Direction::Up => y -= steps,
        }
    }
    outline.push((x, y));
    outline
}

fn lagoon_area(outline: &[(i64, i64)]) -> u64 {
    // Shoelace formula
    let inner_area = outline
        .windows(2)
        .map(|w| {
            // if y1 != y2, then x1 == x2 and the area is 0
            // Otherwise, y1 == y2 => y1 + y2 == 2(y1), so we can half it here
            let (x1, y1) = w[0];
            let (x2, _) = w[1];
            (x2 - x1) * y1
        })
        .sum::<i64>()
        .unsigned_abs();
    // Add a border of width 0.5 as we're using integer coordinates
    let border_area = outline
        .windows(2)
        .map(|w| {
            let (x1, y1) = w[0];
            let (x2, y2) = w[1];
            x1.abs_diff(x2) + y1.abs_diff(y2)
        })
        .sum::<u64>()
        / 2
        + 1; // Add 1 to account for the 4 extra convex corners
    inner_area + border_area
}

#[allow(dead_code)]
pub fn part1() -> u64 {
    let input = parse_input1();
    let outline = get_outline(&input);
    lagoon_area(&outline)
}

fn parse_input1() -> Vec<(Direction, i64)> {
    INPUT
        .lines()
        .map(|line| {
            let mut parts = line.split(' ');
            let d = match parts.next().unwrap() {
                "R" => Direction::Right,
                "D" => Direction::Down,
                "L" => Direction::Left,
                "U" => Direction::Up,
                _ => unreachable!(),
            };
            let steps = parts.next().unwrap().parse().unwrap();
            (d, steps)
        })
        .collect()
}

#[allow(dead_code)]
pub fn part2() -> u64 {
    let input = parse_input2();
    let outline = get_outline(&input);
    lagoon_area(&outline)
}

fn parse_input2() -> Vec<(Direction, i64)> {
    INPUT
        .lines()
        .map(|line| {
            let color = line.split(' ').nth(2).unwrap();
            let d = match color.chars().nth(7).unwrap() {
                '0' => Direction::Right,
                '1' => Direction::Down,
                '2' => Direction::Left,
                '3' => Direction::Up,
                _ => unreachable!(),
            };
            let steps = i64::from_str_radix(&color[2..7], 16).unwrap();
            (d, steps)
        })
        .collect()
}
