use itertools::Itertools;

use aoc::prelude::*;

fn main() {
    let sheet: Vec<Vec<u32>> =
        stdin_lines().map(|line| numbers(line)).collect();

    // Part 1
    let mut sum = 0;
    for row in &sheet {
        sum += row.iter().max().unwrap() - row.iter().min().unwrap();
    }
    println!("{sum}");

    // Part 2
    let mut sum = 0;
    for row in &sheet {
        sum += row
            .iter()
            .tuple_combinations()
            .find_map(|(a, b)| {
                let (a, b) = (a.min(b), b.max(a));
                ((b % a) == 0).then_some(b / a)
            })
            .unwrap();
    }
    println!("{sum}");
}
