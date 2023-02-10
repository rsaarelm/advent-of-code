use aoc::prelude::*;

fn redistribute(banks: &mut [i32]) {
    let max_idx = banks
        .iter()
        .enumerate()
        .rev()
        .max_by_key(|x| x.1)
        .unwrap()
        .0;
    let n = banks[max_idx];
    banks[max_idx] = 0;
    for i in (max_idx + 1)..(max_idx + 1 + n as usize) {
        banks[i % banks.len()] += 1;
    }
}

fn main() {
    let input: Vec<i32> = stdin_string()
        .split_whitespace()
        .map(|a| a.parse().unwrap())
        .collect();

    // Seen values.
    let mut seen = HashSet::from_iter([input.clone()]);
    // Current redistributed state.
    let mut state = input.clone();

    // First repeat value.
    let mut first_repeat = None;
    // Index of first repeat value.
    let mut j = 0;

    for i in 1.. {
        redistribute(&mut state);

        // Part 2
        if first_repeat.as_ref() == Some(&state) {
            println!("{}", i - j);
            break;
        }

        // Part 1
        if first_repeat.is_none() && seen.contains(&state) {
            first_repeat = Some(state.clone());
            j = i;
            println!("{i}");
        }

        seen.insert(state.clone());
    }
}
