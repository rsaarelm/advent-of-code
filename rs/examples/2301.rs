use aoc::prelude::*;

fn digit1(s: &str) -> Option<u32> {
    s.chars().next().and_then(|c| c.to_digit(10))
}

fn digit2(s: &str) -> Option<u32> {
    match s {
        s if s.starts_with("one") => Some(1),
        s if s.starts_with("two") => Some(2),
        s if s.starts_with("three") => Some(3),
        s if s.starts_with("four") => Some(4),
        s if s.starts_with("five") => Some(5),
        s if s.starts_with("six") => Some(6),
        s if s.starts_with("seven") => Some(7),
        s if s.starts_with("eight") => Some(8),
        s if s.starts_with("nine") => Some(9),
        s => digit1(s),
    }
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
