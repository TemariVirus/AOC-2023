mod day10;

use std::time::Instant;

fn main() {
    let start = Instant::now();
    let result = day10::part2();
    let end = Instant::now();

    println!("Result: {}", result);
    println!("Time: {:?}", end.duration_since(start));
}
