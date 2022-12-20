use aoc::prelude::*;
use glam::IVec2;
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    str::FromStr,
};

const BOUNDARY: u32 = 9;

fn neighbors(p: IVec2) -> impl Iterator<Item = IVec2> {
    [[1i32, 0], [0, 1], [-1, 0], [0, -1]]
        .iter()
        .map(move |&d| p + IVec2::from(d))
}

/// Generic graph fill.
fn fill<N: Clone + Eq + Hash>(
    seed: N,
    neighbors: impl Fn(&N) -> Vec<N>,
) -> impl Iterator<Item = N> {
    let mut open: HashSet<_> = Some(seed).into_iter().collect();
    let mut seen = HashSet::new();
    std::iter::from_fn(move || {
        if let Some(elt) = open.pop() {
            seen.insert(elt.clone());

            for neighbor in neighbors(&elt) {
                if !seen.contains(&neighbor) {
                    open.insert(neighbor);
                }
            }

            Some(elt)
        } else {
            None
        }
    })
}

// All points, neighbors_fn

struct Map {
    data: HashMap<IVec2, u32>,
}

impl Map {
    pub fn get(&self, pos: IVec2) -> u32 {
        *self.data.get(&pos).unwrap_or(&BOUNDARY)
    }

    pub fn iter(&self) -> impl Iterator<Item = (IVec2, u32)> + '_ {
        self.data
            .iter()
            .filter(|&(_, &n)| n < BOUNDARY)
            .map(|(&p, &v)| (p, v))
    }

    /// Neighbors function that does not cross `BOUNDARY` cells.
    pub fn neighbors_fn<'a>(&'a self) -> impl Fn(&IVec2) -> Vec<IVec2> + 'a {
        move |&p| neighbors(p).filter(|&n| self.get(n) != BOUNDARY).collect()
    }
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data = s
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| {
                        (
                            IVec2::new(x as i32, y as i32),
                            c.to_digit(10).unwrap(),
                        )
                    })
                    .collect::<Vec<(IVec2, u32)>>()
            })
            .flatten()
            .collect();

        Ok(Map { data })
    }
}

fn main() {
    let map: Map = from_stdin();

    // 1
    let mut n = 0;
    for (p, a) in map.iter() {
        if neighbors(p).any(|n| map.get(n) <= a) {
            continue;
        }
        n += a + 1
    }
    println!("{}", n);

    // 2
    let mut basin_sizes = Vec::new();
    let mut open_points: HashSet<IVec2> = map.iter().map(|(p, _)| p).collect();

    while !open_points.is_empty() {
        let basin: HashSet<_> =
            fill(open_points.pop().unwrap(), map.neighbors_fn()).collect();
        open_points = open_points.difference(&basin).cloned().collect();
        basin_sizes.push(basin.len());
    }

    basin_sizes.sort();
    println!("{}", basin_sizes.iter().rev().take(3).product::<usize>());
}
