use std::ops::Range;

const INPUT: &str = include_str!("day05.txt");

#[derive(Debug)]
struct Map(Vec<RangeMap>);

impl Map {
    fn map_single(&self, n: u64) -> u64 {
        self.0.iter().find_map(|map| map.map_single(n)).unwrap_or(n)
    }

    fn map(&self, ranges: Vec<Range<u64>>) -> Vec<Range<u64>> {
        let mut mapped_ranges = Vec::new();
        let leftover = self.0.iter().fold(ranges, |ranges, map| {
            ranges
                .iter()
                .flat_map(|range| {
                    let (mapped, left, right) = map.map(range.to_owned());
                    if mapped.start < mapped.end {
                        mapped_ranges.push(mapped);
                    }
                    [left, right]
                })
                // Filter out empty ranges
                .filter(|r| r.start < r.end)
                .collect()
        });
        mapped_ranges.extend(leftover);
        mapped_ranges
    }
}

#[derive(Debug)]
struct RangeMap {
    src: u64,
    dest: u64,
    len: u64,
}

impl RangeMap {
    fn map_single(&self, n: u64) -> Option<u64> {
        if n >= self.src && n < self.src + self.len {
            Some(n - self.src + self.dest)
        } else {
            None
        }
    }

    /// Returns (mapped, leftover1, leftover2)
    fn map(&self, range: Range<u64>) -> (Range<u64>, Range<u64>, Range<u64>) {
        let start = self.src;
        let end = self.src + self.len;

        if range.start >= end || range.end <= start {
            // No overlap
            (0..0, range, 0..0)
        } else if range.start >= start && range.end <= end {
            // Range complely inside map
            let map_start = range.start - start + self.dest;
            let map_end = range.end - start + self.dest;
            (map_start..map_end, 0..0, 0..0)
        } else if range.start <= start && range.end >= end {
            // Map complely inside range
            (
                self.dest..self.dest + self.len,
                range.start..start,
                end..range.end,
            )
        } else {
            // Partial overlap
            let map_start = range.start.max(start) - start + self.dest;
            let map_end = range.end.min(end) - start + self.dest;
            let leftover_start = if range.start < start {
                range.start
            } else {
                end
            };
            let leftover_end = if range.end > end { range.end } else { start };
            (map_start..map_end, leftover_start..leftover_end, 0..0)
        }
    }
}

fn parse_input() -> (Vec<u64>, Vec<Map>) {
    let mut parts = INPUT.split("\n\n");
    let seeds = parts
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let str_to_range = |line: &str| {
        let mut nums = line.split_whitespace().map(|s| s.parse().unwrap());
        RangeMap {
            dest: nums.next().unwrap(),
            src: nums.next().unwrap(),
            len: nums.next().unwrap(),
        }
    };
    let maps = parts
        .map(|part| Map(part.lines().skip(1).map(str_to_range).collect()))
        .collect();
    (seeds, maps)
}

#[allow(dead_code)]
pub fn part1() -> u64 {
    let (seeds, maps) = parse_input();
    seeds
        .iter()
        .map(|&seed| maps.iter().fold(seed, |s, map| map.map_single(s)))
        .min()
        .unwrap()
}

#[allow(dead_code)]
pub fn part2() -> u64 {
    let (seeds, maps) = parse_input();
    let seeds = seeds.chunks(2).map(|nums| nums[0]..nums[0] + nums[1]);
    seeds
        .flat_map(|seed| maps.iter().fold(vec![seed], |s, map| map.map(s)))
        .map(|r| r.start)
        .min()
        .unwrap()
}
