use itertools::Itertools;

use aoc::prelude::*;

fn mix(n: i64, x: i64) -> i64 {
    (n ^ x) % 16777216
}

fn prices(mut n: i64) -> impl Iterator<Item = i64> {
    std::iter::from_fn(move || {
        let ret = Some(n);
        n = mix(n, n * 64);
        n = mix(n, n / 32);
        n = mix(n, n * 2048);
        ret
    })
}

fn deltas(
    i: impl Iterator<Item = i64>,
) -> impl Iterator<Item = (i64, [i64; 4])> {
    i.map(|x| x % 10)
        .tuple_windows()
        .map(|(a, b, c, d, e)| (e, [b - a, c - b, d - c, e - d]))
}

fn main() {
    let input: Vec<i64> = stdin_lines_as().collect();

    // P1
    println!(
        "{}",
        input
            .iter()
            .map(|&n| prices(n).nth(2000).unwrap())
            .sum::<i64>()
    );

    // P2
    let mut profits: HashMap<[i64; 4], i64> = Default::default();
    for &n in &input {
        let mut seen_deltas = HashSet::default();

        // We're looking for 2000 changes for P2, so need 2001 initial items.
        for (price, ds) in deltas(prices(n).take(2001)) {
            // Only register a pattern once per seller.
            if seen_deltas.contains(&ds) {
                continue;
            }
            seen_deltas.insert(ds);

            *profits.entry(ds).or_default() += price;
        }
    }

    println!("{}", profits.values().max().unwrap());
}
