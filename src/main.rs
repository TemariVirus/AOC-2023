mod day01;

use std::time::Instant;

fn main() {
    let start = Instant::now();
    let result = day01::part1();
    let end = Instant::now();

    println!("Result: {}", result);
    println!("Time: {:?}", end.duration_since(start));
}