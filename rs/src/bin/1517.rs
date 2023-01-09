use aoc::prelude::*;

fn main() {
    let input: Vec<usize> = stdin_lines_as().collect();

    // bins[i]: number of groups of i containers that fit 150 liters.
    let mut bins = vec![0; input.len()];

    // Encode subsets with binary numbers.
    for i in 0u64..(1 << input.len()) {
        let cap: usize = input
            .iter()
            .enumerate()
            .filter_map(|(j, e)| ((1 << j) & i != 0).then_some(e))
            .sum();
        if cap == 150 {
            bins[i.count_ones() as usize] += 1;
        }
    }

    println!("{}", bins.iter().sum::<usize>());

    for i in bins {
        if i > 0 {
            println!("{i}");
            break;
        }
    }
}
