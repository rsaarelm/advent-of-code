use std::sync::LazyLock;

use aoc::prelude::*;
use itertools::Itertools;
use memoize::memoize;

fn main() {
    let mut input: Vec<(usize, Vec<IVec2>)> = Vec::new();
    for line in stdin_lines() {
        let code: usize = line[..3].parse().unwrap();
        let seq = line
            .chars()
            .filter_map(|c| NUMPAD.get(&c))
            .copied()
            .collect();
        input.push((code, seq));
    }

    for n in [2, 25] {
        println!(
            "{}",
            input
                .iter()
                .map(|(code, seq)| code
                    * expansion_cost(NUM_POSNS.clone(), seq, n))
                .sum::<usize>()
        );
    }
}

fn expansion_cost(
    valid: Vec<IVec2>,
    seq: &[IVec2],
    layers_left: usize,
) -> usize {
    std::iter::once(&ivec2(0, 0))
        .chain(seq.iter())
        .tuple_windows()
        .map(|(&a, &b)| segment_cost(valid.clone(), a, b, layers_left))
        .sum::<usize>()
}

#[memoize]
fn segment_cost(
    valid: Vec<IVec2>,
    p1: IVec2,
    p2: IVec2,
    layers_left: usize,
) -> usize {
    if layers_left == 0 {
        // Tap to the new position and press A, and we're done if there are no
        // more layers.
        return (p2 - p1).taxi_len() as usize + 1;
    }

    // L-shaped moves can expand in two ways, pick the one that costs less.

    let delta = p2 - p1;
    let mut vals = [usize::MAX, usize::MAX];

    for (val, axis) in vals.iter_mut().zip([ivec2(1, 0), ivec2(0, 1)]) {
        let v1 = delta * axis;
        let v2 = delta - v1;

        if (v1 == ivec2(0, 0) && v2 != ivec2(0, 0))
            || !valid.contains(&(p1 + v1))
        {
            continue;
        }

        let mut seq = Vec::new();

        let dir = v1.signum();
        for _ in 0..v1.element_sum().abs() {
            seq.push(DIRPAD[&dir]);
        }

        let dir = v2.signum();
        for _ in 0..v2.element_sum().abs() {
            seq.push(DIRPAD[&dir]);
        }

        // End by pushing A.
        seq.push(ivec2(0, 0));

        *val = expansion_cost(DIR_POSNS.clone(), &seq, layers_left - 1);
    }

    vals[0].min(vals[1])
}

static DIRPAD: LazyLock<HashMap<IVec2, IVec2>> = LazyLock::new(|| {
    HashMap::from_iter([
        (ivec2(0, -1), ivec2(-1, 0)),
        (ivec2(-1, 0), ivec2(-2, 1)),
        (ivec2(0, 1), ivec2(-1, 1)),
        (ivec2(1, 0), ivec2(0, 1)),
    ])
});

static DIR_POSNS: LazyLock<Vec<IVec2>> = LazyLock::new(|| {
    vec![
        ivec2(-1, 0),
        ivec2(0, 0),
        ivec2(-2, 1),
        ivec2(-1, 1),
        ivec2(0, 1),
    ]
});

static NUMPAD: LazyLock<HashMap<char, IVec2>> = LazyLock::new(|| {
    HashMap::from_iter([
        ('A', ivec2(0, 0)),
        ('0', ivec2(-1, 0)),
        ('1', ivec2(-2, -1)),
        ('2', ivec2(-1, -1)),
        ('3', ivec2(0, -1)),
        ('4', ivec2(-2, -2)),
        ('5', ivec2(-1, -2)),
        ('6', ivec2(0, -2)),
        ('7', ivec2(-2, -3)),
        ('8', ivec2(-1, -3)),
        ('9', ivec2(0, -3)),
    ])
});

static NUM_POSNS: LazyLock<Vec<IVec2>> =
    LazyLock::new(|| NUMPAD.values().copied().collect());
