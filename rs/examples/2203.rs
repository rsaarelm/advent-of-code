use aoc::prelude::*;
use itertools::Itertools;

fn priority(c: char) -> u32 {
    let c = c as u32;
    if c >= 'a' as u32 && c <= 'z' as u32 {
        c - 'a' as u32 + 1
    } else if c >= 'A' as u32 && c <= 'Z' as u32 {
        c - 'A' as u32 + 27
    } else {
        panic!("Bad item");
    }
}

fn main() {
    let items: Vec<String> = stdin_lines().collect();

    let mut score = 0;
    for i in &items {
        let split = i.len() / 2;
        let fst: HashSet<_> = i[..split].chars().collect();
        let snd: HashSet<_> = i[split..].chars().collect();

        let shared = fst.intersection(&snd).next().unwrap();
        score += priority(*shared);
    }

    println!("{}", score);

    let mut score = 0;
    for group in &items.iter().chunks(3) {
        let badge = group
            .map(|c| c.chars().collect::<HashSet<_>>())
            .reduce(|a, b| a.intersection(&b).copied().collect::<HashSet<_>>())
            .unwrap()
            .into_iter()
            .next()
            .unwrap();
        score += priority(badge);
    }

    println!("{}", score);
}
