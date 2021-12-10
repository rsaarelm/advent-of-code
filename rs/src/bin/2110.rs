use aoc::prelude::*;
use std::{convert::TryFrom, fmt};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Elt(u8);

const ELT_CHARS: [char; 8] = ['(', '[', '{', '<', ')', ']', '}', '>'];

impl TryFrom<char> for Elt {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        if let Some(i) = ELT_CHARS.iter().position(|&p| p == value) {
            Ok(Elt(i as u8))
        } else {
            Err(())
        }
    }
}

impl fmt::Display for Elt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", ELT_CHARS[self.0 as usize])
    }
}

impl Elt {
    fn pair(self) -> Elt {
        if self.0 < 4 {
            Elt(self.0 + 4)
        } else {
            Elt(self.0 - 4)
        }
    }

    fn is_opening(self) -> bool {
        self.0 < 4
    }

    fn score_1(self) -> u64 {
        [3, 57, 1197, 25137][self.0 as usize % 4]
    }

    fn score_2(self) -> u64 {
        ((self.0 % 4) + 1) as u64
    }
}

#[derive(Debug)]
enum Status {
    Valid,
    Incomplete(Vec<Elt>),
    Corrupt(Vec<Elt>),
}

impl Status {
    fn compute(chunk: impl Iterator<Item = Elt>) -> Status {
        let mut stack = Vec::new();

        let mut mismatches = Vec::new();

        for e in chunk {
            if e.is_opening() {
                stack.push(e.pair());
            } else {
                if e != stack.pop().unwrap() {
                    mismatches.push(e);
                }
            }
        }

        if !mismatches.is_empty() {
            Status::Corrupt(mismatches)
        } else if !stack.is_empty() {
            stack.reverse();
            Status::Incomplete(stack)
        } else {
            Status::Valid
        }
    }

    fn score_1(&self) -> u64 {
        if let Status::Corrupt(elts) = self {
            elts.iter().map(|e| e.score_1()).sum()
        } else {
            0
        }
    }

    fn score_2(&self) -> Option<u64> {
        match self {
            Status::Corrupt(_) => None,
            Status::Incomplete(elts) => Some(elts.iter().fold(0, |a, b| a * 5 + b.score_2())),
            _ => Some(0),
        }
    }
}

fn main() {
    let mut data: Vec<Status> = Vec::new();
    for line in stdin_lines() {
        let status = Status::compute(line.chars().map(|c| Elt::try_from(c).unwrap()));
        data.push(status);
    }

    // 1
    println!("{}", data.iter().map(|a| a.score_1()).sum::<u64>());

    // 2
    let mut scores: Vec<u64> = data.iter().filter_map(|a| a.score_2()).collect();
    scores.sort();
    println!("{}", scores[scores.len() / 2]);
}
