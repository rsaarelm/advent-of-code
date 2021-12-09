use aoc::prelude::*;
use glam::IVec2;
use itertools::Itertools;
use std::{
    collections::{BTreeSet, HashMap, HashSet},
    hash::Hash,
    str::FromStr,
};

const BOUNDARY: u32 = 9;

#[inline(always)]
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
        if open.is_empty() {
            return None;
        }
        let elt = open.iter().next().cloned().unwrap();
        open.remove(&elt);

        seen.insert(elt.clone());

        for neighbor in neighbors(&elt) {
            if !seen.contains(&neighbor) {
                open.insert(neighbor);
            }
        }

        Some(elt)
    })
}

struct Map {
    data: HashMap<IVec2, u32>,
    dim: IVec2,
}

impl Map {
    pub fn get(&self, pos: IVec2) -> u32 {
        *self.data.get(&pos).unwrap_or(&BOUNDARY)
    }

    pub fn all_points(&self) -> impl Iterator<Item = IVec2> {
        (0..self.dim[1])
            .cartesian_product(0..self.dim[0])
            .map(|(y, x)| IVec2::new(x, y))
    }

    /// Neighbors function that does not cross `BOUNDARY` cells.
    pub fn neighbors_fn<'a>(&'a self) -> impl Fn(&IVec2) -> Vec<IVec2> + 'a {
        move |&p| neighbors(p).filter(|&n| self.get(n) != BOUNDARY).collect()
    }
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut w = 0;
        let mut h = 0;
        let data = s
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| {
                        let (x, y) = (x as i32, y as i32);
                        w = w.max(x + 1);
                        h = h.max(y + 1);
                        (IVec2::new(x, y), c.to_digit(10).unwrap())
                    })
                    .collect::<Vec<(IVec2, u32)>>()
            })
            .flatten()
            .collect();

        let dim = [w, h].into();
        Ok(Map { data, dim })
    }
}

fn main() {
    let map: Map = stdin_string().parse().unwrap();

    // 1
    let mut n = 0;
    for p in map.all_points() {
        let a = map.get(p);
        if neighbors(p).any(|n| map.get(n) <= a) {
            continue;
        }
        n += a + 1
    }
    println!("{}", n);

    // 2
    // Use BTreeSets for deduplication because lazy.
    // (BTreeSet can contain BTreeSets, HashSet can't contain HashSets.)
    let regions: BTreeSet<BTreeSet<_>> = map
        .all_points()
        .filter(|&p| map.get(p) != BOUNDARY)
        .map(|p| {
            fill(p, map.neighbors_fn())
                // Can't BTreeSet IVec2s but can BTreeSet arrays.
                .map(|v| <[i32; 2]>::from(v))
                .collect::<BTreeSet<_>>()
        })
        .collect();

    // Top 3
    println!(
        "{}",
        regions
            .iter()
            .map(|x| x.len() as i32)
            .sorted_by_key(|x| -x)
            .take(3)
            .product::<i32>()
    );
}
