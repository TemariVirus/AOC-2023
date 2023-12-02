const INPUT: &str = include_str!("day02.txt");

#[allow(dead_code)]
pub fn part1() -> u32 {
    INPUT
        .lines()
        .filter_map(|game| {
            let (id, rounds) = game.split_once(": ").unwrap();
            let id: u32 = id[5..].parse().unwrap();

            let mut rounds = rounds.split("; ");
            if rounds.all(is_round_possible) {
                Some(id)
            } else {
                None
            }
        })
        .sum()
}

fn is_round_possible(game: &str) -> bool {
    const RED_CAP: u32 = 12;
    const GREEN_CAP: u32 = 13;
    const BLUE_CAP: u32 = 14;

    let mut parts = game.split(", ");
    parts.all(|p| {
        let (count, color) = p.split_once(' ').unwrap();
        let count: u32 = count.parse().unwrap();
        match color {
            "red" => count <= RED_CAP,
            "green" => count <= GREEN_CAP,
            "blue" => count <= BLUE_CAP,
            _ => unreachable!(),
        }
    })
}

#[allow(dead_code)]
pub fn part2() -> u32 {
    INPUT
        .lines()
        .map(|game| {
            let (_, rounds) = game.split_once(": ").unwrap();

            let mins = rounds.split("; ").fold((0u32, 0u32, 0u32), update_min);
            mins.0 * mins.1 * mins.2
        })
        .sum()
}

fn update_min(min: (u32, u32, u32), value: &str) -> (u32, u32, u32) {
    value.split(", ").fold(min, |acc, part| {
        let (count, color) = part.split_once(' ').unwrap();
        let count: u32 = count.parse().unwrap();
        match color {
            "red" => (acc.0.max(count), acc.1, acc.2),
            "green" => (acc.0, acc.1.max(count), acc.2),
            "blue" => (acc.0, acc.1, acc.2.max(count)),
            _ => unreachable!(),
        }
    })
}
