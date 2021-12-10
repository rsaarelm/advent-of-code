use aoc::prelude::*;
use std::convert::TryFrom;

#[derive(Copy, Clone, Debug)]
enum Bracket {
    // Value is corresponding closing char.
    Open(char),
    // Value is score index of bracket pair.
    Close(u8),
}

const fn bracket_table() -> [Option<Bracket>; 128] {
    let mut ret = [None; 128];
    ret['(' as usize] = Some(Bracket::Open(')'));
    ret[')' as usize] = Some(Bracket::Close(0));
    ret['[' as usize] = Some(Bracket::Open(']'));
    ret[']' as usize] = Some(Bracket::Close(1));
    ret['{' as usize] = Some(Bracket::Open('}'));
    ret['}' as usize] = Some(Bracket::Close(2));
    ret['<' as usize] = Some(Bracket::Open('>'));
    ret['>' as usize] = Some(Bracket::Close(3));
    ret
}

// Compile-time look-up table for char values.
const BRACKET_TABLE: [Option<Bracket>; 128] = bracket_table();

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Elt(char);

impl TryFrom<char> for Elt {
    type Error = ();

    fn try_from(c: char) -> Result<Self, Self::Error> {
        if BRACKET_TABLE[c as usize].is_some() {
            Ok(Elt(c))
        } else {
            Err(())
        }
    }
}

impl Elt {
    fn closing_pair(self) -> Option<Elt> {
        if let Some(Bracket::Open(c)) = BRACKET_TABLE[self.0 as usize] {
            Some(Elt(c))
        } else {
            None
        }
    }

    fn rank(self) -> Option<usize> {
        if let Some(Bracket::Close(a)) = BRACKET_TABLE[self.0 as usize] {
            Some(a as usize)
        } else {
            None
        }
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
            if let Some(c) = e.closing_pair() {
                stack.push(c);
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
            elts.iter()
                .filter_map(|e| e.rank())
                .map(|rank| [3, 57, 1197, 25137][rank])
                .sum()
        } else {
            0
        }
    }

    fn score_2(&self) -> Option<u64> {
        match self {
            Status::Corrupt(_) => None,
            Status::Incomplete(elts) => Some(
                elts.iter()
                    .filter_map(|e| e.rank())
                    .fold(0, |a, b| a * 5 + (b as u64) + 1),
            ),
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
