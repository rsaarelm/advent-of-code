use aoc::prelude::*;

fn digit1(s: &str) -> Option<u32> {
    s.chars().next().and_then(|c| c.to_digit(10))
}

fn digit2(s: &str) -> Option<u32> {
    [
        "💩", "one", "two", "three", "four", "five", "six", "seven", "eight",
        "nine",
    ]
    .into_iter()
    .enumerate()
    .find_map(|(n, w)| s.starts_with(w).then_some(n as u32))
    .or_else(|| digit1(s))
}

fn main() {
    let lines = stdin_lines().collect::<Vec<_>>();

    for f in [digit1, digit2] {
        let mut n = 0;

        for line in &lines {
            let a = suffixes(line).find_map(f).unwrap();
            let b = suffixes(line).filter_map(f).last().unwrap();
            n += a * 10 + b;
        }

        println!("{n}");
    }
}
