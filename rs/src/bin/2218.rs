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

    let volume_min =
        droplets.iter().copied().reduce(IVec3::min).unwrap() - IVec3::splat(1);
    let volume_max =
        droplets.iter().copied().reduce(IVec3::max).unwrap() + IVec3::splat(2);

    let open: HashSet<IVec3> = dijkstra_map(
        |&p| {
            let droplets = &droplets; // safe to move.
            SPACE_6.iter().filter_map(move |&d| {
                let n = d + p;
                if !droplets.contains(&n)
                    && p.cmpge(volume_min).all()
                    && p.cmplt(volume_max).all()
                {
                    Some(n)
                } else {
                    None
                }
            })
        },
        volume_min,
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
