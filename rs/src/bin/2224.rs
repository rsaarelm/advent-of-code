use std::str::FromStr;

use aoc::prelude::*;
use rustc_hash::{FxHashMap, FxHashSet};

#[derive(Debug)]
struct Valley {
    blizzards: FxHashMap<IVec2, IVec2>,
    frames: Vec<FxHashSet<IVec2>>,
    bounds: Range2<i32, i32>,
}

impl Valley {
    fn start(&self) -> IVec2 {
        ivec2(0, -1)
    }

    fn end(&self) -> IVec2 {
        self.bounds.max::<IVec2>() - ivec2(1, 0)
    }

    fn grow(&mut self) {
        let t = self.frames.len() as i32;
        self.frames.push(
            self.blizzards
                .iter()
                .map(|(pos, dir)| self.bounds % (*pos + *dir * t))
                .collect(),
        )
    }

    pub fn get(&self, ts: IVec3) -> bool {
        let t = ts.z as usize;
        // Pattern repeats after w * h.
        let t = t % self.bounds.area() as usize;

        self.frames[t].contains(&ts.xy())
    }

    /// Search using 3D space-time coordinates.
    fn neighbors(&self, ts: IVec3) -> impl Iterator<Item = IVec3> + '_ {
        DIR_4
            .iter()
            .copied()
            .map(|d| d.extend(1))
            .chain(Some(ivec3(0, 0, 1)))
            .map(move |d| ts + d)
            .filter(|&p| {
                (self.bounds.contains(p.xy()) && !self.get(p))
                    || p.xy() == self.end()
                    || p.xy() == self.start()
            })
    }
}

impl FromStr for Valley {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (mut x2, mut y2) = (0, 0);

        let mut blizzards = FxHashMap::default();

        for (y, line) in s.lines().enumerate() {
            // Make starting point zero for modular arithmetic of moving
            // blizzards.
            let y = y as i32 - 1;
            if y == -1 {
                // Top wall
                continue;
            } else if line.starts_with("##") {
                y2 = y;
                // Bottom wall.
                break;
            }
            for (x, c) in line.chars().enumerate() {
                let x = x as i32 - 1;
                if x == -1 {
                    // Left wall.
                    debug_assert!(c == '#');
                    continue;
                }
                if c == '#' {
                    debug_assert!(x2 == 0 || x2 == x);
                    x2 = x;
                    // Right wall.
                    break;
                }

                let p = ivec2(x, y);
                match c {
                    '>' => {
                        blizzards.insert(p, DIR_4[RIGHT]);
                    }
                    'v' => {
                        blizzards.insert(p, DIR_4[DOWN]);
                    }
                    '<' => {
                        blizzards.insert(p, DIR_4[LEFT]);
                    }
                    '^' => {
                        blizzards.insert(p, DIR_4[UP]);
                    }
                    _ => {}
                }
            }
        }

        let mut ret = Valley {
            blizzards,
            frames: Default::default(),
            bounds: area((x2, y2)),
        };
        for _ in 0..ret.bounds.area() {
            ret.grow();
        }

        Ok(ret)
    }
}

fn main() {
    let valley: Valley = stdin_string().parse().unwrap();
    let mut t = 0;

    t +=
        dijkstra_map(|&p| valley.neighbors(p), valley.start().extend(t as i32))
            .find(|(p, _)| p.xy() == valley.end())
            .unwrap()
            .1;
    println!("{t}");

    t += dijkstra_map(|&p| valley.neighbors(p), valley.end().extend(t as i32))
        .find(|(p, _)| p.xy() == valley.start())
        .unwrap()
        .1;
    t +=
        dijkstra_map(|&p| valley.neighbors(p), valley.start().extend(t as i32))
            .find(|(p, _)| p.xy() == valley.end())
            .unwrap()
            .1;
    println!("{t}");
}
