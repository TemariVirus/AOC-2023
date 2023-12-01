const INPUT: &str = include_str!("day01.txt");
const DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

pub fn part1() -> u32 {
    INPUT
        // Split into lines
        .split('\n')
        // Convert and filter out non-digits
        .map(|line| line.chars().filter_map(|c| c.to_digit(10)))
        // Get first and last digit
        .map(|mut digits| {
            let first = digits.next().unwrap();
            first * 10 + digits.last().unwrap_or(first)
        })
        // Sum
        .sum()
}

pub fn part2() -> u32 {
    let digits_chars = DIGITS.map(|d| d.chars().collect::<Vec<_>>());

    let lines = INPUT
        .split('\n')
        .map(|line| line.chars().collect::<Vec<_>>());
    let mut sum = 0;
    for line in lines {
        let mut i = 0;
        let mut first = None;
        let mut last = None;
        while i < line.len() {
            let number = 'number: {
                if line[i].is_numeric() {
                    line[i].to_digit(10)
                } else {
                    for (j, d) in digits_chars.iter().enumerate() {
                        if line[i..].starts_with(d) {
                            break 'number Some(j as u32 + 1);
                        }
                    }
                    None
                }
            };
            i += 1;

            if number.is_none() {
                continue;
            }
            if first.is_none() {
                first = number;
            }
            last = number;
        }
        if let (Some(first), Some(last)) = (first, last) {
            sum += first * 10 + last;
        }
    }
    sum
}
