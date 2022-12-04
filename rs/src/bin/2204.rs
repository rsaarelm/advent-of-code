use std::convert::TryInto;

use aoc::prelude::*;
use regex::Regex;

fn main() {
    // Can't use prelude's fixed_numbers because the hyphens confuse it.
    let number = Regex::new(r"\d+").unwrap();

    let pairs: Vec<[u32; 4]> = stdin_lines()
        .map(|line| {
            number
                .find_iter(&line)
                .map(|s| s.as_str().parse().unwrap())
                .collect::<Vec<u32>>()
                .try_into()
                .unwrap()
        })
        .collect();

    let mut covers = 0;
    for [a, b, x, y] in &pairs {
        if (x >= a && y <= b) || (a >= x && b <= y) {
            covers += 1;
        }
    }
    println!("{}", covers);

    let mut laps = 0;
    for [a, b, x, y] in &pairs {
        if b < x || y < a {
            continue;
        }
        laps += 1;
    }
    println!("{}", laps);
}
