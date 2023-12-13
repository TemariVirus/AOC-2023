const INPUT: &str = include_str!("day13.txt");

fn parse_input() -> Vec<(Vec<u32>, Vec<u32>)> {
    INPUT
        .split("\n\n")
        .map(|pattern| {
            let rows = pattern
                .lines()
                .map(|row| {
                    row.bytes()
                        .enumerate()
                        .map(|(i, c)| match c {
                            b'#' => 1 << i,
                            b'.' => 0,
                            _ => unreachable!(),
                        })
                        .sum::<u32>()
                        | (1 << row.len()) // Add leading 1
                })
                .collect::<Vec<_>>();
            let width = u32::BITS - rows[0].leading_zeros() - 1;
            let cols = (0..width)
                .map(|i| {
                    rows.iter()
                        .map(|row| (row >> i) & 1)
                        .fold(0, |acc, x| (acc << 1) | x)
                        | (1 << rows.len()) // Add leading 1
                })
                .collect::<Vec<_>>();
            (rows, cols)
        })
        .collect()
}

#[allow(dead_code)]
pub fn part1() -> usize {
    parse_input()
        .iter()
        .map(|(rows, cols)| 100 * find_mirror(rows) + find_mirror(cols))
        .sum()
}

fn find_mirror(pattern: &[u32]) -> usize {
    'outer: for i in 1..pattern.len() {
        if pattern[i] != pattern[i - 1] {
            continue;
        }

        let len = i.min(pattern.len() - i);
        for j in 1..len {
            if pattern[i + j] != pattern[i - j - 1] {
                continue 'outer;
            }
        }
        return i;
    }
    0
}

#[allow(dead_code)]
pub fn part2() -> usize {
    parse_input()
        .into_iter()
        .map(|(mut rows, mut cols)| {
            100 * find_mirror_with_smudge(&mut rows) + find_mirror_with_smudge(&mut cols)
        })
        .sum()
}

fn find_mirrors_all(pattern: &[u32]) -> Vec<usize> {
    let mut mirrors = Vec::new();
    'outer: for i in 1..pattern.len() {
        if pattern[i] != pattern[i - 1] {
            continue;
        }

        let len = i.min(pattern.len() - i);
        for j in 1..len {
            if pattern[i + j] != pattern[i - j - 1] {
                continue 'outer;
            }
        }
        mirrors.push(i);
    }
    mirrors
}

fn find_mirror_with_smudge(pattern: &mut [u32]) -> usize {
    let mirror = find_mirror(pattern);

    for i in 0..pattern.len() {
        for j in 0..u32::BITS - pattern[i].leading_zeros() - 1 {
            pattern[i] ^= 1 << j;
            let mirrors = find_mirrors_all(pattern);
            pattern[i] ^= 1 << j;

            for m in mirrors {
                if m != mirror {
                    return m;
                }
            }
        }
    }
    0
}
