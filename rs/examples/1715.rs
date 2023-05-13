use aoc::prelude::*;

fn gen(mut seed: u64, factor: u64) -> impl Iterator<Item = u64> {
    std::iter::from_fn(move || {
        let ret = Some(seed);
        seed *= factor;
        seed %= 0x7fffffff;
        ret
    })
}

fn main() {
    let input: Vec<u64> =
        parsed_stdin_lines(r"Generator . starts with (.*)").collect();

    let a = || gen(input[0], 16807);
    let b = || gen(input[1], 48271);

    let mut matches = 0;
    for (a, b) in a().zip(b()).take(40_000_000) {
        if a & 0xffff == b & 0xffff {
            matches += 1;
        }
    }
    println!("{matches}");

    let mut matches = 0;
    for (a, b) in a()
        .filter(|a| a % 4 == 0)
        .zip(b().filter(|b| b % 8 == 0))
        .take(5_000_000)
    {
        if a & 0xffff == b & 0xffff {
            matches += 1;
        }
    }
    println!("{matches}");
}
