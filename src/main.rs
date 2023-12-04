mod day04;

use std::time::Instant;

fn main() {
    let start = Instant::now();
    let result = day04::part2();
    let end = Instant::now();

    println!("Result: {}", result);
    println!("Time: {:?}", end.duration_since(start));
}
