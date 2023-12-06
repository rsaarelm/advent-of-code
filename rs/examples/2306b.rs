use std::fmt::Write;

use aoc::prelude::*;

fn wins(time: u64, record: u64) -> usize {
    // c: record
    // b: available time
    // x: time spent powering up
    //
    // x (b - x) > c
    // -x^2 + bx - c = 0
    //
    // x = [b +/- sqrt(b^2 + 4c)] / 2

    let b = time as f64;
    let c = record as f64 - 0.0;

    let d = (b * b - 4.0 * c).sqrt();

    let p1 = (b - d) / 2.0;
    let p2 = (b + d) / 2.0;

    // Adjust the numbers if they're exact integers.
    let p1 = if p1 == p1.round() {
        p1 + 1.0
    } else {
        p1.ceil()
    };

    let p2 = if p2 == p2.round() {
        p2 - 1.0
    } else {
        p2.floor()
    };

    (p2 - p1 + 1.0) as usize
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
