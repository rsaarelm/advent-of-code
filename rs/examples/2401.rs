use aoc::prelude::*;

fn main() {
    let input: Vec<[i64; 2]> = stdin_lines().map(fixed_numbers).collect();

    let mut a: Vec<i64> = input.iter().map(|[a, _]| *a).collect();
    let mut b: Vec<i64> = input.iter().map(|[_, b]| *b).collect();
    a.sort();
    b.sort();

    println!("{}", a.iter().zip(b.iter()).map(|(a, b)| (a-b).abs()).sum::<i64>());

    println!("{}", a.iter().map(|&x| x * b.iter().filter(|&&y| y == x).count() as i64).sum::<i64>());
}

