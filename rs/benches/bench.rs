#![feature(test)]
extern crate test;

use aoc::prelude::*;
use rand::{rngs::SmallRng, Rng, SeedableRng};
use test::Bencher;

#[bench]
fn bench_astar(b: &mut Bencher) {
    let mut rng = SmallRng::from_seed([123; 32]);
    let bounds = area(1000, 1000);
    let mut grid = vec![false; bounds.volume() as usize];

    for i in grid.iter_mut() {
        // Fudge up a sampling rate that generates a terrain that's still
        // traversable. RNG seed is fixed so this should work every time
        // unless the RNG algorithm changes.
        if rng.gen_range(0..8) == 0 {
            *i = true;
        }
    }

    let neighbors = |p: &IVec2| {
        neighbors_4(*p)
            .filter(|&p| bounds.contains(p) && !grid[bounds.index_of(p)])
    };

    b.iter(|| {
        astar_search(
            neighbors,
            |pos| (*pos - ivec2(999, 999)).taxi_len() as f32,
            ivec2(0, 0),
        )
        .unwrap()
    });
}
