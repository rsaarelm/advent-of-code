use aoc::prelude::*;

#[memoize]
fn fish_at_zero_spawns(days: i32) -> u64 {
    if days < 0 {
        0
    } else {
        1 + fish_at_zero_spawns(days - 7) + fish_at_zero_spawns(days - 9)
    }
}

fn main() {
    let input: Vec<i32> = numbers(&stdin_lines().next().unwrap());

    for n in [80, 256] {
        println!(
            "{}",
            input
                .iter()
                .map(|&x| 1 + fish_at_zero_spawns(n - 1 - x))
                .sum::<u64>()
        );
    }
}
