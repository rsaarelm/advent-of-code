use aoc::prelude::*;

fn main() {
    let input: Vec<i32> = numbers(stdin_lines().next().unwrap());
    let (min, max) = (*input.iter().min().unwrap(), *input.iter().max().unwrap());

    println!(
        "{}",
        (min..=max)
            .map(|x| input.iter().map(|p| (x - p).abs()).sum::<i32>())
            .min()
            .unwrap()
    );

    println!(
        "{}",
        (min..=max)
            .map(|x| input
                .iter()
                .map(|p| {
                    let n = (x - p).abs();
                    n * (n + 1) / 2
                })
                .sum::<i32>())
            .min()
            .unwrap()
    );
}
