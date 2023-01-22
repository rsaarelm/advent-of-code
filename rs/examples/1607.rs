use itertools::Itertools;

use aoc::prelude::*;

fn main() {
    let mut count_1 = 0;
    let mut count_2 = 0;
    for line in stdin_lines() {
        // Part 1 match state.
        let mut matched = false;
        let mut is_valid = true;

        // Part 2 match state.
        let mut aba_matches = HashSet::default();
        let mut bab_matches = HashSet::default();

        // Assumptions:
        // No invalid bracketing in input.
        // No nested bracket sections.
        for (i, e) in line.split(['[', ']']).enumerate() {
            for (a1, b1, b2, a2) in e.chars().tuple_windows() {
                if a1 == a2 && b1 == b2 && a1 != b1 {
                    if i % 2 == 1 {
                        is_valid = false;
                        break;
                    } else {
                        matched = true;
                    }
                }
            }

            for (a1, b, a2) in e.chars().tuple_windows() {
                if a1 == a2 && a1 != b {
                    if i % 2 == 0 {
                        aba_matches.insert((a1, b));
                    } else {
                        bab_matches.insert((b, a1));
                    }
                }
            }
        }

        if matched && is_valid {
            count_1 += 1;
        }

        if !aba_matches.is_disjoint(&bab_matches) {
            count_2 += 1;
        }
    }

    println!("{count_1}");
    println!("{count_2}");
}
