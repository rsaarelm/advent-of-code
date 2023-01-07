use aoc::prelude::*;
use itertools::Itertools;

fn main() {
    let ds: Vec<usize> = stdin_lines()
        .map(|line| fixed_numbers::<usize, 1>(line)[0])
        .collect();

    // Chart lists each pair once, length is Σi for element count n-1.
    // Solve n from chart length.
    // Σi = (n-1) * ((n-1) + 1) / 2
    // n = (√(8 * Σi + 1) + 1) / 2
    let n = ((1.0 + (8.0 * ds.len() as f32 + 1.0).sqrt()) / 2.0) as usize;

    // Rely on chart listing the items in logical order so we can ignore the
    // names.
    let mut d = vec![vec![0; n]; n];
    let mut k = 0;
    for i in 0..n {
        for j in (i + 1)..n {
            d[i][j] = ds[k];
            d[j][i] = ds[k];
            k += 1;
        }
    }

    // Find shortest and longest out of all routes.
    let mut mx = 0;
    let mut mn = ds.iter().sum::<usize>();

    for p in (0..n).permutations(n) {
        let x = p
            .iter()
            .zip(p.iter().skip(1))
            .map(|(a, b)| d[*a][*b])
            .sum::<usize>();
        mx = mx.max(x);
        mn = mn.min(x);
    }
    println!("{mn}");
    println!("{mx}");
}
