use aoc::prelude::*;
use itertools::Itertools;

fn score(h: &[Vec<i32>], p: &[usize]) -> i32 {
    p.iter()
        .cycle()
        .tuple_windows()
        .take(p.len())
        .map(|(lhs, x, rhs)| h[*x][*lhs] + h[*x][*rhs])
        .sum()
}

fn main() {
    let nums: Vec<i32> = stdin_lines()
        .map(|line|
            if line.contains("lose") { -1 } else { 1 } *
            fixed_numbers::<i32, 1>(line)[0])
        .collect();

    // Exploit input regularity so we can ignore names. There are n people and
    // each person gets values for the (n-1) other people.

    // a = (n - 1) * n
    // n = (√(4 * a + 1) + 1) / 2
    let n = (((4.0 * nums.len() as f32 + 1.0).sqrt() + 1.0) / 2.0) as usize;

    // Add +1 space for the indifferent main character.
    let mut h = vec![vec![0; n + 1]; n + 1];
    let mut k = 0;
    for i in 0..n {
        for j in 0..(n - 1) {
            let j = if j < i { j } else { j + 1 };
            h[i][j] = nums[k];
            k += 1;
        }
    }

    for n in [n, n + 1] {
        println!(
            "{}",
            (0..n).permutations(n).map(|p| score(&h, &p)).max().unwrap()
        );
    }
}
