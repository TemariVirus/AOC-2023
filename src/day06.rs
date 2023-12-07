const INPUT: &str = include_str!("day06.txt");

// Let x be the time the button is held, t be the time of the race, and d be the distance traveled.
// Then,
// speed = x
// time_left = t - x
// d = (t - x)x
//   = tx - x^2
//
// We want to find the range of values of x where:
// d > d_record
// tx - x^2 > d_record
// x^2 - tx + d_record < 0
//
// Using the quadratic formula, we find that x^2 - tx + d_record = 0 has roots:
// r_1 = (t - sqrt(discriminant)) / 2
// r_2 = (t + sqrt(discriminant)) / 2
// where discriminant = t^2 - 4d_record
//
// So we want the length of the range [ceil(r_1), floor(r_2)], right?
// Well, r_1 and r_2 are not valid solutions as you have to go FURTHER than the record distance.
// Hence,
// start = floor(r_1) + 1
// end = ceil(r_2) - 1
// range = end - start + 1
//       = ceil(r_2) - floor(r_1) - 1
fn winnable_range(time: u64, record_dist: u64) -> u64 {
    let discriminant = time * time - 4 * record_dist;
    let discriminant_sqrt = (discriminant as f64).sqrt();
    let root1 = ((time as f64) - discriminant_sqrt) / 2.0;
    let root2 = ((time as f64) + discriminant_sqrt) / 2.0;
    root2.ceil() as u64 - root1.floor() as u64 - 1
}

#[allow(dead_code)]
pub fn part1() -> u64 {
    let mut lines = INPUT
        .lines()
        .map(|line| line.split_once(':').unwrap().1.split_whitespace());
    lines
        .next()
        .unwrap()
        .zip(lines.next().unwrap())
        .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
        .map(|(time, dist)| winnable_range(time, dist))
        .product()
}

#[allow(dead_code)]
pub fn part2() -> u64 {
    let mut lines = INPUT.lines().map(|line| {
        line.split_once(':')
            .unwrap()
            .1
            .replace(' ', "")
            .parse()
            .unwrap()
    });
    let time = lines.next().unwrap();
    let dist = lines.next().unwrap();
    winnable_range(time, dist)
}
