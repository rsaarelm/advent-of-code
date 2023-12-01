use aoc::prelude::*;

fn crunch(tokens: &[(&str, i32)], mut input: &str) -> i32 {
    let mut nums = Vec::new();
    'scan: while !input.is_empty() {
        for (t, o) in tokens {
            if input.starts_with(t) {
                input = &input[1..];
                nums.push(o);
                continue 'scan;
            }
        }
        input = &input[1..];
    }
    if nums.is_empty() {
        return 0;
    }

    nums[0] * 10 + nums[nums.len() - 1]
}

fn main() {
    let lines = stdin_lines().collect::<Vec<_>>();

    let p1 = [
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ];

    println!("{}", lines.iter().map(|s| crunch(&p1, s)).sum::<i32>());

    let p2 = [
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    println!("{}", lines.iter().map(|s| crunch(&p2, s)).sum::<i32>());
}
