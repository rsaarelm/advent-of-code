use aoc::prelude::*;
use itertools::Itertools;
use lazy_static::lazy_static;
use std::{collections::BTreeSet, convert::TryInto, fmt};

//   aaaa
//  b    c
//  b    c
//   dddd
//  e    f
//  e    f
//   gggg
//
// Wire bit set equals number segment lit

const DIGITS: [Wire; 10] = [
    //     gfedcba
    Wire(0b1110111), // 0
    Wire(0b0100100), // 1
    Wire(0b1011101), // 2
    Wire(0b1101101), // 3
    Wire(0b0101110), // 4
    Wire(0b1101011), // 5
    Wire(0b1111011), // 6
    Wire(0b0100101), // 7
    Wire(0b1111111), // 8
    Wire(0b1101111), // 9
];

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct Wire(u8);

impl Wire {
    pub fn new(a: impl AsRef<str>) -> Wire {
        Wire(
            a.as_ref()
                .chars()
                .map(|c| (1 << (c as usize - 'a' as usize)))
                .sum(),
        )
    }

    pub fn len(self) -> u32 {
        self.0.count_ones()
    }
}

impl fmt::Debug for Wire {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:07b}", self.0)
    }
}

impl TryInto<u32> for Wire {
    type Error = ();

    fn try_into(self) -> Result<u32, Self::Error> {
        for i in 0..10 {
            if self == DIGITS[i as usize] {
                return Ok(i);
            }
        }

        Err(())
    }
}

/// Wire permutator.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Key([u8; 7]);

impl std::ops::Deref for Key {
    type Target = [u8; 7];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::Mul<Key> for Wire {
    type Output = Wire;

    fn mul(self, rhs: Key) -> Self::Output {
        // Rewire each set bit according to key.
        Wire(
            (0..7)
                .map(|i| {
                    if self.0 & (1 << i) != 0 {
                        1 << rhs[i]
                    } else {
                        0
                    }
                })
                .sum(),
        )
    }
}

lazy_static! {
    static ref ALL_KEYS: Vec<Key> = (0..7)
        .permutations(7)
        .map(|p| Key(p.as_slice().try_into().unwrap()))
        .collect();
    static ref VALID_WIRES: BTreeSet<Wire> = DIGITS.iter().cloned().collect();
}

fn main() {
    let mut wirings: Vec<Vec<Wire>> = Vec::new();
    let mut digits: Vec<Vec<Wire>> = Vec::new();
    for line in stdin_lines() {
        match line.split(" | ").collect::<Vec<&str>>().as_slice() {
            [w, d] => {
                wirings.push(w.split(' ').map(|a| Wire::new(a)).collect());
                digits.push(d.split(' ').map(|a| Wire::new(a)).collect());
            }
            _ => panic!(),
        }
    }

    // 1
    println!(
        "{}",
        digits
            .iter()
            .map(|d| d
                .iter()
                .filter(|a| a.len() == 2 || a.len() == 3 || a.len() == 4 || a.len() == 7)
                .count())
            .sum::<usize>()
    );

    // 2
    let mut acc = 0;
    'top: for (ws, ds) in wirings.iter().zip(&digits) {
        // brute force go brrr
        for &k in ALL_KEYS.iter() {
            let rewired: BTreeSet<Wire> = ws.iter().map(|&w| w * k).collect();

            if rewired == *VALID_WIRES {
                let num: u32 = ds
                    .iter()
                    .rev()
                    .enumerate()
                    .map(|(i, &d)| {
                        let d: u32 = (d * k).try_into().unwrap();
                        d * 10u32.pow(i as u32)
                    })
                    .sum();
                acc += num;
                continue 'top;
            }
        }
        panic!("No matches found!");
    }
    println!("{}", acc);
}
