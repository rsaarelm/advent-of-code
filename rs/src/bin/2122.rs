use aoc::prelude::*;
use itertools::Itertools;
use std::collections::BTreeSet;

/// Non-uniform grid where each cell is a variable-sized axis-aligned box in
/// the uniform integer grid space.
struct ChunkyGrid {
    xs: Vec<i32>,
    ys: Vec<i32>,
    zs: Vec<i32>,
}

impl ChunkyGrid {
    /// Construct a minimal `ChunkyGrid` where each uniform space box from the
    /// input will fit snugly.
    pub fn new<'a>(boxes: impl Iterator<Item = &'a [i32; 6]>) -> ChunkyGrid {
        let mut xp = BTreeSet::new();
        let mut yp = BTreeSet::new();
        let mut zp = BTreeSet::new();
        for [x0, x1, y0, y1, z0, z1] in boxes {
            // Input ranges are end-incusive, grid is end-exclusive so add 1
            // to end ranges.
            xp.insert(*x0);
            xp.insert(*x1 + 1);

            yp.insert(*y0);
            yp.insert(*y1 + 1);

            zp.insert(*z0);
            zp.insert(*z1 + 1);
        }
        // Deduplicated vecs.
        let xs: Vec<i32> = xp.into_iter().collect();
        let ys: Vec<i32> = yp.into_iter().collect();
        let zs: Vec<i32> = zp.into_iter().collect();

        ChunkyGrid { xs, ys, zs }
    }

    /// Convert uniform space point to chunk space point.
    pub fn map(&self, pos: [i32; 3]) -> [usize; 3] {
        [
            self.xs.iter().position(|&a| a == pos[0]).unwrap(),
            self.ys.iter().position(|&a| a == pos[1]).unwrap(),
            self.zs.iter().position(|&a| a == pos[2]).unwrap(),
        ]
    }

    /// Compute uniform space volume of a chunk space point.
    pub fn volume(&self, pos: [usize; 3]) -> i64 {
        (self.xs[pos[0] + 1] - self.xs[pos[0]]) as i64
            * (self.ys[pos[1] + 1] - self.ys[pos[1]]) as i64
            * (self.zs[pos[2] + 1] - self.zs[pos[2]]) as i64
    }

    /// Enumerate chunk space points in an uniform space box.
    pub fn cells(&self, realspace_cube: &[i32; 6]) -> impl Iterator<Item = [usize; 3]> {
        let [x0, x1, y0, y1, z0, z1] = *realspace_cube;
        // Map to gridspace, ranges to end-exclusive.
        let [cx0, cy0, cz0] = self.map([x0, y0, z0]);
        let [cx1, cy1, cz1] = self.map([x1 + 1, y1 + 1, z1 + 1]);

        // XXX: Figure out how the hell you get .multi_cartesian_product to work...
        (cx0..cx1)
            .cartesian_product((cy0..cy1).cartesian_product(cz0..cz1))
            .map(|(x, (y, z))| [x, y, z])
    }
}

fn main() {
    let mut data: Vec<(bool, [i32; 6])> = Vec::new();
    for line in stdin_lines() {
        let is_on = line.split(' ').next().unwrap().len() == 2;
        let cube: [i32; 6] = fixed_numbers(line);
        data.push((is_on, cube));
    }

    let grid = ChunkyGrid::new(data.iter().map(|(_, p)| p));

    eprintln!(
        "Chunky grid: {} x {} x {} = {} cells",
        grid.xs.len(),
        grid.ys.len(),
        grid.zs.len(),
        grid.xs.len() * grid.ys.len() * grid.zs.len()
    );

    let filters: [Box<dyn Fn(&[i32; 6]) -> bool>; 2] = [
        // Part 1.
        Box::new(|cube| {
            cube[1] >= -50
                && cube[3] >= -50
                && cube[5] >= -50
                && cube[0] <= 50
                && cube[2] <= 50
                && cube[4] <= 50
        }),
        // Part 2.
        Box::new(|_| true),
    ];

    for valid in filters {
        let mut state = BTreeSet::new();
        for (bit, cube) in &data {
            if !valid(cube) {
                continue;
            }
            for p in grid.cells(cube) {
                if *bit {
                    state.insert(p);
                } else {
                    state.remove(&p);
                }
            }
        }
        println!("{}", state.into_iter().map(|c| grid.volume(c)).sum::<i64>());
    }
}
