use aoc::prelude::*;

mod knot_hash;
use knot_hash::knot_hash;

fn reverse(state: &mut Vec<u32>, pos: usize, n: usize) {
    let len = state.len();
    for i in 0..(n / 2) {
        let (j, k) = ((pos + i) % len, (pos + n - 1 - i) % len);
        (state[j], state[k]) = (state[k], state[j]);
    }
}

fn main() {
    let s = stdin_string();
    let input: Vec<usize> = numbers(&s);
    let mut state: Vec<u32> = (0..=255).collect();

    let mut pos = 0;
    for (skip, &n) in input.iter().enumerate() {
        reverse(&mut state, pos, n);
        pos += skip + n;
    }
    println!("{}", state[0] * state[1]);

    // Part 2

    // Unit test
    assert_eq!(
        bytes_to_hex(&knot_hash(&Vec::new())),
        "a2582a3a0e66e6e86e3812dcb672a272"
    );

    println!("{}", bytes_to_hex(&knot_hash(s.as_bytes())));
}
