use aoc::prelude::*;
use memoize::memoize;

#[memoize]
fn can_build(towels: Vec<String>, design: String) -> usize {
    if design.is_empty() {
        return 1;
    }
    let mut ret = 0;
    for t in &towels {
        if let Some(suffix) = design.strip_prefix(t) {
            ret += can_build(towels.clone(), suffix.to_string());
        }
    }

    ret
}

fn main() {
    let mut lines = stdin_lines();
    let towels: Vec<String> = lines
        .next()
        .unwrap()
        .split(", ")
        .map(|a| a.to_string())
        .collect();
    lines.next().unwrap();
    let designs: Vec<String> = lines.map(|a| a.to_string()).collect();

    println!(
        "{}",
        designs
            .iter()
            .filter(|d| can_build(towels.clone(), d.to_string()) > 0)
            .count()
    );
    println!(
        "{}",
        designs
            .iter()
            .map(|d| can_build(towels.clone(), d.to_string()))
            .sum::<usize>()
    );
}
