use aoc::prelude::*;
use itertools::Itertools;

const ADD: usize = 0;
const MUL: usize = 1;
const CAT: usize = 2;

fn find(a: i64, eqn: &[i64], n_ops: usize) -> bool {
    // Generate all permutations of ops from ADD to n_ops-1 for the current
    // equation.
    'search: for ops in
        std::iter::repeat_n(0..n_ops, eqn.len() - 1).multi_cartesian_product()
    {
        let mut ret = eqn[0];

        for (i, e) in eqn.iter().skip(1).enumerate() {
            match ops[i] {
                ADD => ret += e,
                MUL => ret *= e,
                CAT => ret = format!("{ret}{e}").parse().unwrap(),
                x => panic!("Bad op {x}"),
            }
            if ret > a {
                continue 'search;
            }
        }

        if ret == a {
            return true;
        }
    }

    false
}

fn main() {
    let input: Vec<Vec<i64>> = stdin_lines().map(numbers).collect();
    for ops in [2, 3] {
        println!(
            "{}",
            input
                .iter()
                .filter_map(|a| find(a[0], &a[1..], ops).then_some(a[0]))
                .sum::<i64>()
        );
    }
}
