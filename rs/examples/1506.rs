use std::str::FromStr;

use aoc::prelude::*;

#[derive(Copy, Clone, Debug)]
enum Op {
    On,
    Off,
    Toggle,
}

use Op::*;

impl Op {
    fn p1(self, n: i64) -> i64 {
        match self {
            On => 1,
            Off => 0,
            // !0 == -1, !-1 == 0
            Toggle => -!(-n),
        }
    }

    fn p2(self, n: i64) -> i64 {
        match self {
            On => n + 1,
            Off => 0.max(n - 1),
            Toggle => n + 2,
        }
    }
}

impl FromStr for Op {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("turn on ") {
            Ok(On)
        } else if s.starts_with("turn off ") {
            Ok(Off)
        } else if s.starts_with("toggle ") {
            Ok(Toggle)
        } else {
            Err(())
        }
    }
}

fn main() {
    let mut ops = Vec::new();
    for line in stdin_lines() {
        let op = Op::from_str(&line).unwrap();

        let [a, b]: [IVec2; 2] = to_ivec2s(numbers(line).into_iter())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        ops.push((op, NRange::from_points_inclusive([a, b])));
    }

    let bounds = volume([1000, 1000]);
    let mut grid_1 = vec![0; bounds.volume() as usize];
    let mut grid_2 = vec![0; bounds.volume() as usize];

    for (op, range) in &ops {
        for p in *range {
            let i = bounds.index_of(p);
            grid_1[i] = op.p1(grid_1[i]);
            grid_2[i] = op.p2(grid_2[i]);
        }
    }

    println!("{}", grid_1.iter().sum::<i64>());
    println!("{}", grid_2.iter().sum::<i64>());
}
