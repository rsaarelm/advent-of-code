use std::ops::RangeInclusive;

use aoc::prelude::*;

fn main() {
    let mut items: Vec<i64> = Vec::new();
    let mut fresh: Vec<RangeInclusive<i64>> = Vec::new();
    for line in stdin_lines() {
        match &numbers(line)[..] {
            [] => continue,
            [a] => items.push(*a),
            [a, b] => fresh.push(*a..=*b),
            _ => panic!(),
        }
    }

    // Merge overlapping ranges.
    fresh.sort_by(|a, b| a.start().cmp(b.start()));
    let mut i = 0;
    while i + 1 < fresh.len() {
        if fresh[i + 1].start() <= fresh[i].end() {
            fresh[i + 1] =
                *fresh[i].start()..=*fresh[i + 1].end().max(fresh[i].end());
            fresh.remove(i);
        } else {
            i += 1;
        }
    }

    // P1
    println!(
        "{}",
        items
            .iter()
            .filter(|i| fresh.iter().any(|f| f.contains(i)))
            .count()
    );

    // P2
    println!(
        "{}",
        fresh.iter().map(|a| a.end() - a.start() + 1).sum::<i64>()
    );
}
