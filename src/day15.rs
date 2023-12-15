use std::iter::repeat_with;

const INPUT: &str = include_str!("day15.txt");

fn hash(input: &[u8]) -> u8 {
    let mut hash: u8 = 0;
    for &byte in input {
        hash = hash.wrapping_add(byte).wrapping_mul(17);
    }
    hash
}

#[allow(dead_code)]
pub fn part1() -> u32 {
    INPUT
        .split(',')
        .map(|s| s.as_bytes())
        .map(|s| hash(s) as u32)
        .sum()
}

#[allow(dead_code)]
pub fn part2() -> usize {
    INPUT
        .split(',')
        .map(|s| s.as_bytes())
        .map(|mut s| {
            let remove = s[s.len() - 1] == b'-';
            let mut focal_length = 0;
            if remove {
                s = &s[..s.len() - 1]
            } else {
                focal_length = s[s.len() - 1] - b'0';
                s = &s[..s.len() - 2]
            }

            (s, hash(s), remove, focal_length)
        })
        .fold(
            repeat_with(Vec::new).take(256).collect(),
            |mut acc: Vec<Vec<(&[u8], u8)>>, (label, idx, remove, focal_length)| {
                let lens_box = &mut acc[idx as usize];
                if remove {
                    for i in 0..lens_box.len() {
                        if lens_box[i].0 == label {
                            lens_box.remove(i);
                            return acc;
                        }
                    }
                } else {
                    for lens in lens_box.iter_mut() {
                        if lens.0 == label {
                            lens.1 = focal_length;
                            return acc;
                        }
                    }
                    lens_box.push((label, focal_length));
                }
                acc
            },
        )
        .iter()
        .enumerate()
        .fold(0, |acc, (i, lens_box)| {
            acc + lens_box
                .iter()
                .enumerate()
                .map(|(j, &(_, focal))| (i + 1) * (j + 1) * focal as usize)
                .sum::<usize>()
        })
}
