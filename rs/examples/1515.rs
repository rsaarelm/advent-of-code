use aoc::prelude::*;
use nalgebra::{DMatrix, DVector};

fn main() {
    let input: Vec<[i64; 5]> = stdin_lines().map(fixed_numbers).collect();
    // Build matrix with ingredients as columns.
    let a = DMatrix::from_fn(5, input.len(), |r, c| input[c][r]);

    // Elements of column vector x are ingredient weights.
    // They must sum to 100, so the vector traces a simplex hypersurface.
    // Set simplex to initial state.
    let mut x = DVector::from(vec![0; input.len()]);
    x[0] = 100;

    let mut p1 = 0;
    let mut p2 = 0;
    loop {
        let result = &a * &x;
        let calories = result[4];
        let score = result.rows(0, 4).map(|c| c.max(0)).product();
        p1 = p1.max(score);
        if calories == 500 {
            p2 = p2.max(score);
        }
        step(&mut x);
        if x[0] == 100 {
            // Looped back to initial state.
            break;
        }
    }

    println!("{p1}");
    println!("{p2}");
}

/// Step into the next simplex configuration.
///
/// - Vector element sum stays constant.
/// - First element balances the rest.
/// - Iterate the rest in lexical order.
fn step(simplex: &mut DVector<i64>) {
    for i in 1..simplex.len() {
        if simplex[0] > 0 {
            simplex[i] += 1;
            simplex[0] -= 1;
            return;
        } else {
            simplex[0] += simplex[i];
            simplex[i] = 0;
        }
    }
}
