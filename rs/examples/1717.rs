use aoc::prelude::*;

fn main() {
    let step: usize = stdin_string().parse().unwrap();

    let mut p = 0;
    let mut buf = vec![0];

    for i in 1..=2017 {
        p = (p + step + 1) % i;
        buf.insert(p, i);
    }
    println!("{}", buf[(p + 1) % buf.len()]);

    // P2
    // The trick: Zero value never moves from index 0.
    let mut p = 0;
    let mut result = 0;
    for i in 1..=50_000_000 {
        p = (p + step + 1) % i;
        if p == 0 {
            result = i;
        }
    }
    println!("{result}");
}
