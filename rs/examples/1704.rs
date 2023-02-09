use aoc::prelude::*;

fn main() {
    let input: Vec<String> = stdin_lines().collect();

    println!(
        "{}",
        input
            .iter()
            .filter(|s| s.split_whitespace().collect::<HashSet<_>>().len()
                == s.split_whitespace().count())
            .count()
    );

    println!(
        "{}",
        input
            .iter()
            .filter(|s| s
                .split_whitespace()
                .map(|w| {
                    let mut w: Vec<char> = w.chars().collect();
                    w.sort();
                    w
                })
                .collect::<HashSet<_>>()
                .len()
                == s.split_whitespace().count())
            .count()
    );
}
