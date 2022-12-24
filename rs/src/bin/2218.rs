use std::collections::HashSet;

use aoc::prelude::*;

fn main() {
    let droplets: HashSet<IVec3> = stdin_lines()
        .map(|line| fixed_numbers::<i32, 3>(&line).into())
        .collect();

    // Part 1

    let mut surf = 0;
    for &cube in &droplets {
        for &d in &SPACE_6 {
            if !droplets.contains(&(cube + d)) {
                surf += 1;
            }
        }
    }
    println!("{surf}");

    // Part 2

    let bounds = NRange::from_points_inclusive(droplets.iter().copied())
        .inflate([1, 1, 1]);

    let open: HashSet<IVec3> = dijkstra_map(
        |&p| {
            let droplets = &droplets; // safe to move.
            SPACE_6.iter().filter_map(move |&d| {
                let n = d + p;
                if !droplets.contains(&n) && bounds.contains(n) {
                    Some(n)
                } else {
                    None
                }
            })
        },
        bounds.min(),
    )
    .map(|(n, _)| n)
    .collect();

    let mut surf = 0;
    for &cube in &droplets {
        for &d in &SPACE_6 {
            if !droplets.contains(&(cube + d)) && open.contains(&(cube + d)) {
                surf += 1;
            }
        }
    }
    println!("{surf}");
}
