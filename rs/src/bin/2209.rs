use aoc::prelude::*;
use derive_deref::Deref;
use std::{collections::HashSet, str::FromStr};

#[derive(Copy, Clone, Deref)]
struct Dir(IVec2);

impl FromStr for Dir {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "R" => Ok(Dir(ivec2(1, 0))),
            "D" => Ok(Dir(ivec2(0, 1))),
            "L" => Ok(Dir(ivec2(-1, 0))),
            "U" => Ok(Dir(ivec2(0, -1))),
            _ => Err(()),
        }
    }
}

fn trace(input: &[(Dir, usize)], rope_len: usize) -> usize {
    let mut rope = vec![ivec2(0, 0); rope_len];
    let mut tail_cover = HashSet::new();

    for &(dir, n) in input {
        for _ in 0..n {
            rope[0] += *dir;

            for i in 1..rope_len {
                let delta = rope[i - 1] - rope[i];
                if delta.abs().max_element() > 1 {
                    rope[i] += delta.signum();
                } else {
                    // Exit early if the rest of the rope isn't being pulled.
                    break;
                }
            }

            tail_cover.insert(rope[rope.len() - 1]);
        }
    }

    tail_cover.len()
}

fn main() {
    let input: Vec<(Dir, usize)> = parsed_stdin_lines(r"^(.) (\d+)$").collect();

    println!("{}", trace(&input, 2));
    println!("{}", trace(&input, 10));
}
