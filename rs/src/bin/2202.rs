use aoc::prelude::*;
use std::convert::TryInto;

fn score(a: u32, b: u32) -> u32 {
    let shape_score = b + 1;

    // RPS: Successor in modulo 3 beats predecessor.
    let win_score = if b == a {
        3 // draw
    } else if b == (a + 1) % 3 {
        6 // win
    } else {
        0 // lose
    };

    shape_score + win_score
}

fn score_1([a, b]: &[char; 2]) -> u32 {
    score(*a as u32 - 'A' as u32, *b as u32 - 'X' as u32)
}

fn score_2([a, b]: &[char; 2]) -> u32 {
    let a = *a as u32 - 'A' as u32;
    let b = match b {
        'X' => (a + 2) % 3, // lose
        'Y' => a,           // draw
        'Z' => (a + 1) % 3, // win
        _ => panic!("unexpected"),
    };

    score(a, b)
}

fn main() {
    let moves = stdin_lines()
        .map(|x| {
            x.split_whitespace()
                .map(|c| c.chars().next().unwrap())
                .collect::<Vec<char>>()
                .try_into()
                .unwrap()
        })
        .collect::<Vec<[char; 2]>>();

    println!("{:?}", moves.iter().map(score_1).sum::<u32>());
    println!("{:?}", moves.iter().map(score_2).sum::<u32>());
}
