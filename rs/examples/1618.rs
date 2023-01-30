use aoc::prelude::*;

fn next(mask: u128, n: u128) -> u128 {
    let mut ret = 0;
    for i in 1..127 {
        if matches!(n >> (i - 1) & 0b111, 0b100 | 0b001 | 0b110 | 0b011) {
            ret |= 1 << i;
        }
    }
    ret & mask
}

fn main() {
    let input = stdin_string();
    assert!(input.len() <= 126);

    let mask: u128 = ((1 << input.len()) - 1) << 1;
    let seed: u128 = input
        .chars()
        .enumerate()
        .filter_map(|(i, c)| (c == '^').then_some(i))
        .map(|i| 1 << (i + 1))
        .sum();

    for y in [40, 400_000] {
        let mut n = seed;
        let safe_spots = std::iter::from_fn(move || {
            let ret = Some(!n & mask);
            n = next(mask, n);
            ret
        });

        println!(
            "{}",
            safe_spots.take(y).map(u128::count_ones).sum::<u32>()
        );
    }
}
