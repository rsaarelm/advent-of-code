use aoc::prelude::*;

fn extrapolate(xs: &[i64]) -> (i64, i64) {
    if xs.iter().all(|&x| x == 0) {
        return (0, 0);
    }

    let simpler: Vec<i64> = xs.windows(2).map(|x| x[1] - x[0]).collect();
    let (a, b) = extrapolate(&simpler);
    (xs[0] - a, xs[xs.len() - 1] + b)
}

fn main() {
    let input: Vec<Vec<i64>> = stdin_lines().map(numbers).collect();
    println!("{}", input.iter().map(|e| extrapolate(e).1).sum::<i64>());
    println!("{}", input.iter().map(|e| extrapolate(e).0).sum::<i64>());
}
