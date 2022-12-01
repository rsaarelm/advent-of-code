use aoc::prelude::*;

fn main() {
    let mut elves: Vec<u32> = vec![0];
    for line in stdin_lines() {
        if let Ok(n) = line.parse::<u32>() {
            let i = elves.len() - 1;
            elves[i] += n;
        } else {
            elves.push(0);
        }
    }

    println!("{}", elves.iter().max().unwrap());

    elves.sort();
    elves.reverse();
    println!("{}", elves[0] + elves[1] + elves[2]);
}
