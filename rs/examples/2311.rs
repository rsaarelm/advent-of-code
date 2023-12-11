use glam::{i64vec2, I64Vec2};

use aoc::prelude::*;

fn expand(coords: &[I64Vec2], age: i64) -> i64 {
    let dim = Rect::from_points_inclusive(coords.iter().copied()).dim();

    // Offsets for X and Y axes of the original coordinate set.
    let mut off = [Vec::new(), Vec::new()];

    for i in 0..2 {
        let mut d = 0;
        off[i] = (0..dim[i])
            .map(|a| {
                if !coords.iter().any(|p| p[i] == a) {
                    d += age - 1;
                }
                d
            })
            .collect();
    }

    // Expand coordinates by offsets.
    let coords_2: Vec<I64Vec2> = coords
        .iter()
        .map(|&p| p + i64vec2(off[0][p.x as usize], off[1][p.y as usize]))
        .collect();

    // Compute pairwise taxicab metric distances.
    let mut n = 0;

    for i in 0..coords_2.len() {
        for j in (i + 1)..coords_2.len() {
            let d = (coords_2[j] - coords_2[i]).abs();
            n += d.x + d.y;
        }
    }

    n
}

fn main() {
    let coords: Vec<I64Vec2> = stdin_grid_iter()
        .filter_map(|(p, c)| {
            (c == '#').then_some(i64vec2(p[0] as i64, p[1] as i64))
        })
        .collect();

    println!("{}", expand(&coords, 2));
    println!("{}", expand(&coords, 1_000_000));
}
