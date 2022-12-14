use std::collections::HashSet;

use aoc::prelude::*;
use glam::{ivec2, IVec2};
use itertools::Itertools;

fn main() {
    let mut walls: HashSet<IVec2> = HashSet::new();
    let mut max_y = 0;

    for line in stdin_lines() {
        let coords: Vec<IVec2> = numbers(&line)
            .into_iter()
            .chunks(2)
            .into_iter()
            .map(|c| {
                let v = c.collect::<Vec<i32>>();
                max_y = max_y.max(v[1]);
                ivec2(v[0], v[1])
            })
            .collect();

        for (i, j) in coords.iter().zip(coords.iter().skip(1)) {
            let mut p = *i;
            let delta = (*j - *i).signum();
            while p != *j {
                walls.insert(p);
                p += delta;
            }
            walls.insert(*j);
        }
    }

    /*
    for y in 0..50 {
        for x in 460..540 {
            if space.contains(&ivec2(x, y)) {
            eprint!("#");
            } else {
                eprint!(".");
            }
        }
        eprintln!();
    }
    */

    // TODO: Separate walls and sand
    let mut pile = walls.clone();

    'sand: for sand in 0.. {
        let mut pos = ivec2(500, 0);
        loop {
            if pos.y > max_y {
                println!("{}", sand);
                break 'sand;
            }

            if !pile.contains(&(pos + ivec2(0, 1))) {
                pos += ivec2(0, 1);
                continue;
            }
            if !pile.contains(&(pos + ivec2(-1, 1))) {
                pos += ivec2(-1, 1);
                continue;
            }
            if !pile.contains(&(pos + ivec2(1, 1))) {
                pos += ivec2(1, 1);
                continue;
            }
            pile.insert(pos);
            break;
        }
    }

    for x in 0..1000 {
        walls.insert(ivec2(x, max_y + 2));
    }

    let mut pile = walls.clone();

    'sand: for sand in 0.. {
        let mut pos = ivec2(500, 0);
        loop {
            if !pile.contains(&(pos + ivec2(0, 1))) {
                pos += ivec2(0, 1);
                continue;
            }
            if !pile.contains(&(pos + ivec2(-1, 1))) {
                pos += ivec2(-1, 1);
                continue;
            }
            if !pile.contains(&(pos + ivec2(1, 1))) {
                pos += ivec2(1, 1);
                continue;
            }
            pile.insert(pos);
            if pos == ivec2(500, 0) {
                println!("{}", sand + 1);
                break 'sand;
            }
            break;
        }
    }
}
