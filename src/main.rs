mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

use std::{fmt::Display, time::Instant};

fn main() {
    let start = Instant::now();

    time(1, 1, day01::part1);
    time(1, 2, day01::part2);
    time(2, 1, day02::part1);
    time(2, 2, day02::part2);
    time(3, 1, day03::part1);
    time(3, 2, day03::part2);
    time(4, 1, day04::part1);
    time(4, 2, day04::part2);
    time(5, 1, day05::part1);
    time(5, 2, day05::part2);
    time(6, 1, day06::part1);
    time(6, 2, day06::part2);
    time(7, 1, day07::part1);
    time(7, 2, day07::part2);
    time(8, 1, day08::part1);
    time(8, 2, day08::part2);
    time(9, 1, day09::part1);
    time(9, 2, day09::part2);
    time(10, 1, day10::part1);
    time(10, 2, day10::part2);
    time(11, 1, day11::part1);
    time(11, 2, day11::part2);
    time(12, 1, day12::part1);
    time(12, 2, day12::part2);
    time(13, 1, day13::part1);
    time(13, 2, day13::part2);
    time(14, 1, day14::part1);
    time(14, 2, day14::part2);
    time(15, 1, day15::part1);
    time(15, 2, day15::part2);
    time(16, 1, day16::part1);
    time(16, 2, day16::part2);
    time(17, 1, day17::part1);
    time(17, 2, day17::part2);
    time(18, 1, day18::part1);
    time(18, 1, day18::part1);
    time(18, 2, day18::part2);
    time(19, 1, day19::part1);
    time(19, 2, day19::part2);
    time(20, 1, day20::part1);
    time(20, 2, day20::part2);
    time(21, 1, day21::part1);
    time(21, 2, day21::part2);
    time(22, 1, day22::part1);
    time(22, 2, day22::part2);
    time(23, 1, day23::part1);
    time(23, 2, day23::part2);
    time(24, 1, day24::part1);
    time(24, 2, day24::part2);
    time(25, 1, day25::part1);

    let end = Instant::now();
    println!("Total Time: {:?}", end.duration_since(start));
}

fn time<T, F: FnOnce() -> T>(day: u8, part: u8, f: F)
where
    T: Display,
{
    let start = Instant::now();
    let result = f();
    let end = Instant::now();

    println!(
        "Day {:>2} Part {}: {:<15} | Time: {:?}",
        day,
        part,
        result,
        end.duration_since(start)
    );
}
