use aoc::prelude::*;

fn main() {
    let mut dial: i64 = 50;

    let mut ends = 0;
    let mut crossings = 0;

    for line in stdin_lines() {
        let (prefix, num) = line.split_at(1);
        let sign = if prefix == "L" { -1 } else { 1 };
        let num: i64 = num.parse::<i64>().unwrap();

        // Fuck it.
        for _ in 0..num {
            dial += sign;
            if dial.rem_euclid(100) == 0 {
                crossings += 1;
            }
        }

        if dial.rem_euclid(100) == 0 {
            ends += 1;
        }
    }

    println!("{ends}");
    println!("{crossings}");
}
