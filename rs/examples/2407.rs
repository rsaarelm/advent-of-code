use aoc::prelude::*;
use itertools::Itertools;

fn find(a: i64, eqn: &[i64], ops: &[Box<dyn Fn(i64, i64) -> i64>]) -> bool {
    // Generate all op permutations for the given equation.
    'search: for ops in
        std::iter::repeat_n(ops, eqn.len() - 1).multi_cartesian_product()
    {
        let mut ret = eqn[0];

        for (i, e) in eqn.iter().skip(1).enumerate() {
            ret = ops[i](ret, *e);
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
    let ops: [Box<dyn Fn(i64, i64) -> i64>; 3] = [
        Box::new(|a, b| a + b),
        Box::new(|a, b| a * b),
        Box::new(|a, b| format!("{a}{b}").parse().unwrap()),
    ];

    let input: Vec<Vec<i64>> = stdin_lines().map(numbers).collect();
    for ops in [&ops[0..2], &ops[0..3]] {
        println!(
            "{}",
            input
                .iter()
                .filter_map(|a| find(a[0], &a[1..], ops).then_some(a[0]))
                .sum::<i64>()
        );
    }
}
