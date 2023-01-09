use aoc::prelude::*;

fn pos(speed: i32, burst: i32, rest: i32, t: i32) -> i32 {
    let period = burst + rest;
    (t / period) * speed * burst + (t % period).min(burst) * speed
}

fn main() {
    let input: Vec<[i32; 3]> = stdin_lines().map(fixed_numbers).collect();

    println!(
        "{}",
        input
            .iter()
            .map(|[s, b, r]| pos(*s, *b, *r, 2503))
            .max()
            .unwrap()
    );

    let mut score = vec![0; input.len()];
    for t in 1..2504 {
        let ps: Vec<i32> =
            input.iter().map(|[s, b, r]| pos(*s, *b, *r, t)).collect();
        let lead = *ps.iter().max().unwrap();
        for (i, s) in score.iter_mut().enumerate() {
            if ps[i] == lead {
                *s += 1;
            }
        }
    }
    println!("{}", score.iter().max().unwrap());
}
