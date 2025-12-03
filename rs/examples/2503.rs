use std::cmp::Reverse;

use aoc::prelude::*;

fn voltage(battery: &str, remain: usize) -> i64 {
    let (idx, &d) = battery.as_bytes().iter().take(battery.len() - remain)
        .enumerate()
        .max_by(|(i, a), (j, b)| (a, Reverse(i)).cmp(&(b, Reverse(j))))
        .unwrap();

    let d = (d - b'0') as i64;

    if remain > 0 {
        d * 10_i64.pow(remain as u32) + voltage(&battery[idx+1..], remain - 1)
    } else {
        d
    }
}

fn main() {
    let input: Vec<String> = stdin_lines().collect();
    println!("{}", input.iter().map(|s| voltage(s, 1)).sum::<i64>());
    println!("{}", input.iter().map(|s| voltage(s, 11)).sum::<i64>());
}
