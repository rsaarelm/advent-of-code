use std::fmt;

use aoc::prelude::*;

// Leave a space at start for shuffled jokers in P2.
const RANK: &str = "?23456789TJQKA";
const JOKER_IDX: usize = 10;

#[derive(Clone, Eq, PartialEq, Debug)]
struct Hand(Vec<usize>);

impl FromStr for Hand {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut a = Vec::new();
        for c in s.chars() {
            a.push(RANK.find(c).unwrap());
        }
        Ok(Hand(a))
    }
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for c in self.0.iter() {
            write!(f, "{}", RANK.chars().nth(*c).unwrap())?;
        }

        Ok(())
    }
}

impl Hand {
    fn key_1(&self) -> (usize, Vec<usize>) {
        let a = match &histogram(self.0.iter())
            .map(|(_, a)| a)
            .collect::<Vec<_>>()[..]
        {
            [1, 1, 1, 1, 1] => 0,
            [2, 1, 1, 1] => 1,
            [2, 2, 1] => 2,
            [3, 1, 1] => 3,
            [3, 2] => 4,
            [4, 1] => 5,
            [5] => 6,
            _ => panic!(),
        };

        (a, self.0.clone())
    }

    fn key_2(&self) -> (usize, Vec<usize>) {
        // Match against joker count and histogram. Jokers push the type up,
        // but the joker count must be zero or one of the histogram numbers.
        let a = match (
            self.0.iter().filter(|&&a| a == JOKER_IDX).count(),
            &histogram(self.0.iter()).map(|(_, a)| a).collect::<Vec<_>>()[..],
        ) {
            (0, [1, 1, 1, 1, 1]) => 0,
            (1, [1, 1, 1, 1, 1]) => 1,

            (0, [2, 1, 1, 1]) => 1,
            (_, [2, 1, 1, 1]) => 3,

            (0, [2, 2, 1]) => 2,
            (1, [2, 2, 1]) => 4,
            (2, [2, 2, 1]) => 5,

            (0, [3, 1, 1]) => 3,
            (_, [3, 1, 1]) => 5,

            (0, [3, 2]) => 4,
            (_, [3, 2]) => 6,

            (0, [4, 1]) => 5,
            (_, [4, 1]) => 6,

            (_, [5]) => 6,
            _ => panic!(),
        };

        // Make jokers rank lower than the other cards.
        let hand = self
            .0
            .iter()
            .map(|&c| if c == JOKER_IDX { 0 } else { c })
            .collect();

        (a, hand)
    }
}

fn main() {
    let input: Vec<(Hand, usize)> = stdin_lines()
        .map(|s| {
            let (a, b) = s.split_once(' ').unwrap();
            (a.parse().unwrap(), b.parse().unwrap())
        })
        .collect();

    let mut p1 = input.clone();
    let mut p2 = input.clone();

    p1.sort_by_key(|(h, _)| h.key_1());
    p2.sort_by_key(|(h, _)| h.key_2());

    for input in [p1, p2] {
        println!(
            "{}",
            input
                .iter()
                .enumerate()
                .map(|(i, (_, n))| (i + 1) * n)
                .sum::<usize>()
        );
    }
}
