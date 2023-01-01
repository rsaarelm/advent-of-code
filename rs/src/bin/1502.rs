use aoc::prelude::*;

fn main() {
    let input: Vec<[i32; 3]> = stdin_lines().map(fixed_numbers).collect();

    println!(
        "{}",
        input
            .iter()
            .map(|[l, w, h]| 2 * l * w
                + 2 * w * h
                + 2 * h * l
                + [l * w, w * h, h * l].into_iter().min().unwrap())
            .sum::<i32>()
    );

    println!(
        "{}",
        input
            .iter()
            .map(|[l, w, h]| [l * 2 + w * 2, w * 2 + h * 2, h * 2 + l * 2]
                .into_iter()
                .min()
                .unwrap()
                + l * w * h)
            .sum::<i32>()
    );
}
