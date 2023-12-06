use std::fmt::Write;

use aoc::prelude::*;

fn wins(time: u64, record: u64) -> usize {
    (1..time)
        .map(|a| (time - a) * a)
        .filter(|&a| a > record)
        .count()
}

fn main() {
    let lines = stdin_lines().collect::<Vec<_>>();
    let t1: Vec<u64> = numbers(&lines[0]);
    let r1: Vec<u64> = numbers(&lines[1]);

    let mut t2 = String::new();
    for t in &t1 {
        let _ = write!(&mut t2, "{t}");
    }
    let t2 = t2.parse::<u64>().unwrap();

    let mut r2 = String::new();
    for r in &r1 {
        let _ = write!(&mut r2, "{r}");
    }
    let r2 = r2.parse::<u64>().unwrap();

    println!(
        "{}",
        t1.iter()
            .zip(r1.iter())
            .map(|(&t, &r)| wins(t, r))
            .product::<usize>()
    );

    println!("{}", wins(t2, r2));
}
