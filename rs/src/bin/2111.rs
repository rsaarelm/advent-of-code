use std::iter;

use aoc::prelude::*;

const PITCH: i64 = 12;

fn pulse(state: &mut [i64], p: i64) {
    let p = {
        if p < 0 || p >= state.len() as i64 {
            return;
        } else {
            p as usize
        }
    };

    state[p] += 1;
    if state[p] == 10 {
        for i in [1, PITCH - 1, PITCH, PITCH + 1] {
            pulse(state, p as i64 + i);
            pulse(state, p as i64 - i);
        }
    }
}

fn reset(state: &mut [i64]) -> usize {
    state
        .iter_mut()
        .map(|n| {
            if *n >= 10 {
                *n = 0;
                1
            } else {
                0
            }
        })
        .sum()
}

fn cycle(state: &mut [i64]) -> usize {
    for i in 0..state.len() {
        pulse(state, i as i64);
    }
    reset(state)
}

fn main() {
    let state: Vec<i64> = stdin_lines()
        // Generate padding at horizontal edges so pitch-based +/-1 update
        // doesn't cycle around to the other side.
        .map(|line| {
            iter::once(i64::MIN)
                .chain(
                    line.chars()
                        .filter_map(|c| c.to_digit(10).map(|c| c as i64)),
                )
                .chain(iter::once(i64::MIN))
                .collect::<Vec<i64>>()
        })
        .flatten()
        .collect();

    // 1
    let mut state_1 = state.clone();
    println!("{}", (0..100).map(|_| cycle(&mut state_1)).sum::<usize>());

    // 2
    let mut state_2 = state.clone();
    println!("{}", (1..).find(|_| cycle(&mut state_2) == 100).unwrap());
}
