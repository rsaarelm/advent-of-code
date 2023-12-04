use aoc::prelude::*;

fn main() {
    let mut cards = Vec::new();
    for line in stdin_lines() {
        let Some((head, tail)) = line.split_once('|') else {
            panic!()
        };
        let mut head: Vec<u32> = numbers(head);
        let tail: Vec<u32> = numbers(tail);
        head.remove(0); // Card number, unneeded.
        cards.push((head, tail));
    }

    let mut wins = vec![0; cards.len()];
    let mut muls = vec![1; cards.len()];

    for (i, (winning, nums)) in cards.iter().enumerate() {
        wins[i] = nums.iter().filter(|&n| winning.contains(n)).count();
        for j in (i + 1)..(i + 1 + wins[i]) {
            muls[j] += muls[i];
        }
    }

    println!(
        "{}",
        wins.iter()
            .map(|&a| if a > 0 { 1 << (a - 1) } else { 0 })
            .sum::<u32>()
    );

    println!("{}", muls.iter().sum::<usize>());
}
