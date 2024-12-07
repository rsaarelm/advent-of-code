use aoc::prelude::*;
use itertools::Itertools;

#[derive(Copy, Clone, Debug)]
pub enum Op {
    Add,
    Mul,
    Cat,
}

use Op::*;

impl Op {
    pub fn apply(self, a: i64, b: i64) -> i64 {
        match self {
            Add => a + b,
            Mul => a * b,
            Cat => format!("{a}{b}").parse().unwrap(),
        }
    }
}

fn find(a: i64, eqn: &[i64], ops: &[Op]) -> bool {
    // Generate all op permuatations for the given equation.
    'search: for ops in
        std::iter::repeat_n(ops, eqn.len() - 1).multi_cartesian_product()
    {
        let mut ret = eqn[0];

        for (i, e) in eqn.iter().skip(1).enumerate() {
            ret = ops[i].apply(ret, *e);
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
    for ops in [vec![Add, Mul], vec![Add, Mul, Cat]] {
        println!(
            "{}",
            input
                .iter()
                .filter_map(|a| find(a[0], &a[1..], &ops).then_some(a[0]))
                .sum::<i64>()
        );
    }
}
