use std::{collections::HashSet, str::FromStr};

use aoc::prelude::*;

#[derive(Default)]
struct Chasm {
    walls: HashSet<Vec2>,
    sand: HashSet<Vec2>,
    max_y: i64,
    has_floor: bool,
}

impl FromStr for Chasm {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut walls = HashSet::new();
        let mut max_y = 0;
        for line in s.lines() {
            let coords: Vec<Vec2> = to_vec2s(numbers(line).into_iter()).collect();

            for (start, end) in coords.iter().zip(coords.iter().skip(1)) {
                let span = *end - *start;
                let dir = span.signum();
                for c in 0..=span.abs().max_element() {
                    let p = *start + dir * c;
                    max_y = max_y.max(p.y);
                    walls.insert(p);
                }
            }
        }

        Ok(Chasm {
            walls,
            max_y,
            ..Default::default()
        })
    }
}

impl Chasm {
    pub fn is_blocked(&self, pos: Vec2) -> bool {
        self.walls.contains(&pos)
            || self.sand.contains(&pos)
            || (self.has_floor && pos.y > self.max_y + 1)
    }

    pub fn add_floor(&mut self) {
        self.has_floor = true;
    }

    pub fn clear_sand(&mut self) {
        self.sand.clear();
    }

    pub fn drop(&mut self, mut pos: Vec2) -> Option<Vec2> {
        loop {
            if self.has_floor && pos.y == self.max_y + 1 {
                // Stopped by floor if it exists.
                self.sand.insert(pos);
                return Some(pos);
            }

            if pos.y > (self.max_y + 2) {
                // Not stopped by walls or floor.
                return None;
            }

            if !self.is_blocked(pos + vec2(0, 1)) {
                pos += vec2(0, 1);
                continue;
            }
            if !self.is_blocked(pos + vec2(-1, 1)) {
                pos += vec2(-1, 1);
                continue;
            }
            if !self.is_blocked(pos + vec2(1, 1)) {
                pos += vec2(1, 1);
                continue;
            }
            self.sand.insert(pos);
            return Some(pos);
        }
    }
}

fn main() {
    let mut chasm: Chasm = stdin_string().parse().unwrap();

    for sand in 0.. {
        if chasm.drop(vec2(500, 0)).is_none() {
            println!("{}", sand);
            break;
        }
    }

    chasm.clear_sand();
    chasm.add_floor();

    println!(
        "{}",
        dijkstra_map(
            |&p| [p + vec2(0, 1), p + vec2(-1, 1), p + vec2(1, 1)]
                .into_iter()
                .filter(|&p| !chasm.is_blocked(p)),
            vec2(500, 0)
        )
        .count()
    );
}
