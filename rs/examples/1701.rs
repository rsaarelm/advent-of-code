use aoc::prelude::*;

fn main() {
    let input: Vec<u32> = stdin_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    for s in [1, input.len() / 2] {
        println!(
            "{}",
            input
                .iter()
                .cycle()
                .zip(input.iter().cycle().skip(s))
                .take(input.len())
                .filter_map(|(a, b)| (a == b).then_some(a))
                .sum::<u32>()
        );
    }
}
