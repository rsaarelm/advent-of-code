use itertools::Itertools;

use aoc::prelude::*;

fn safe_pair(sign: i32, a: i32, b: i32) -> bool {
    (b - a).signum() == sign && (1..=3).contains(&(a - b).abs())
}

fn is_safe(seq: &[i32]) -> bool {
    let sign = (seq[1] - seq[0]).signum();
    seq.iter()
        .tuple_windows()
        .all(|(a, b)| safe_pair(sign, *a, *b))
}

fn is_safe_2(seq: &[i32]) -> bool {
    // Brute force goes brrr.
    for i in 0..seq.len() {
        let mut test = seq.to_owned();
        test.remove(i);
        if is_safe(&test) {
            return true;
        }
    }

    false
}

fn main() {
    let input: Vec<Vec<i32>> = stdin_lines().map(numbers).collect();

    println!("{}", input.iter().filter(|x| is_safe(x)).count());
    println!("{}", input.iter().filter(|x| is_safe_2(x)).count());
}
