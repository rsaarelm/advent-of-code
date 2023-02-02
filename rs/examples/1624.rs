use aoc::prelude::*;
use itertools::Itertools;

fn main() {
    let (bounds, grid) = stdin_grid();
    let points: HashMap<usize, IVec2> = bounds
        .into_iter()
        .filter_map(|p| {
            grid[bounds.idx(p)]
                .to_digit(10)
                .map(|d| (d as usize, p.into()))
        })
        .collect();

    let mut d = vec![vec![0; points.len()]; points.len()];

    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let dist = grid_astar(&points[&i], &points[&j], |&p| {
                neighbors_4(p).filter(|&p| grid[bounds.idx(p)] != '#')
            })
            .unwrap()
            .len()
                - 1;
            d[i][j] = dist;
            d[j][i] = dist;
        }
    }

    for ret in [false, true] {
        // Travelling salesman the points.
        let mut n = usize::MAX;
        for mut p in (1..points.len()).permutations(points.len() - 1) {
            if ret {
                p.push(0);
            }
            let len: usize = Some(0)
                .into_iter()
                .chain(p.iter().copied())
                .zip(p.iter().copied())
                .map(|(a, b)| d[a][b])
                .sum();
            n = n.min(len);
        }

        println!("{n}");
    }
}
