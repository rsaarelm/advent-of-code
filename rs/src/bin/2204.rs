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

    println!("{}", pairs.iter().filter(|[a, b, x, y]| (x >= a && y <= b) || (a >= x && b <= y)).count());
    println!("{}", pairs.iter().filter(|[a, b, x, y]| !(b < x || y < a)).count());
}
