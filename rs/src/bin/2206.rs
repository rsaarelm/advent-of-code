use aoc::prelude::*;
use std::collections::HashSet;

fn main() {
    let input = stdin_string();

    for n in [4, 14] {
        println!(
            "{}",
            input
                .as_bytes()
                .windows(n)
                .enumerate()
                .filter(|(_, a)| a.iter().collect::<HashSet<_>>().len() == n)
                .map(|(i, _)| i + n)
                .next()
                .unwrap()
        );
    }
}
