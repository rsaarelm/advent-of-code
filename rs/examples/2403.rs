use regex::Regex;

use aoc::prelude::*;

fn run(p2: bool, text: &str) -> i32 {
    // Parameter-less ops have dummy capture groups to make the regex valid.
    let op = Regex::new(r"(mul)\((\d+),(\d+)\)|(do)\(()()\)|(don't)\(()()\)")
        .unwrap();
    let mut is_active = true;
    let mut ret = 0;

    for (_, [op, a, b]) in op.captures_iter(text).map(|c| c.extract()) {
        match op {
            "do" => is_active = true,
            "don't" => is_active = false,
            "mul" if is_active || !p2 => {
                let (a, b): (i32, i32) =
                    (a.parse().unwrap(), b.parse().unwrap());
                ret += a * b;
            }
            _ => {}
        }
    }

    ret
}

fn main() {
    let text = stdin_string();
    println!("{}", run(false, &text));
    println!("{}", run(true, &text));
}
