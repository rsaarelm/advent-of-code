use aoc::prelude::*;
use itertools::Itertools;
use std::{collections::BTreeSet, convert::TryInto, fmt};

//   aaaa
//  b    c
//  b    c
//   dddd
//  e    f
//  e    f
//   gggg
//
// Digit byte bit set equals number segment lit.

const DIGITS: [Digit; 10] = [
    //      gfedcba
    Digit(0b1110111), // 0
    Digit(0b0100100), // 1
    Digit(0b1011101), // 2
    Digit(0b1101101), // 3
    Digit(0b0101110), // 4
    Digit(0b1101011), // 5
    Digit(0b1111011), // 6
    Digit(0b0100101), // 7
    Digit(0b1111111), // 8
    Digit(0b1101111), // 9
];

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct Digit(u8);

impl Digit {
    pub fn new(a: impl AsRef<str>) -> Digit {
        Digit(
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

impl fmt::Debug for Digit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:07b}", self.0)
    }
}

impl TryInto<u32> for Digit {
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

/// Wire permutator for digits.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Wiring([u8; 7]);

impl std::ops::Deref for Wiring {
    type Target = [u8; 7];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::Mul<Wiring> for Digit {
    type Output = Digit;

    fn mul(self, rhs: Wiring) -> Self::Output {
        // Rewire each set bit according to wiring.
        Digit(
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

fn main() {
    let mut wirings: Vec<Vec<Digit>> = Vec::new();
    let mut numbers: Vec<Vec<Digit>> = Vec::new();
    for line in stdin_lines() {
        match line.split(" | ").collect::<Vec<&str>>().as_slice() {
            [w, d] => {
                wirings.push(w.split(' ').map(|a| Digit::new(a)).collect());
                numbers.push(d.split(' ').map(|a| Digit::new(a)).collect());
            }
            _ => panic!(),
        }
    }

    // 1
    println!(
        "{}",
        numbers
            .iter()
            .map(|d| d
                .iter()
                .filter(|a| a.len() == 2 || a.len() == 3 || a.len() == 4 || a.len() == 7)
                .count())
            .sum::<usize>()
    );

    // 2
    let all_wirings: Vec<Wiring> = (0..7)
        .permutations(7)
        .map(|p| Wiring(p.as_slice().try_into().unwrap()))
        .collect();

    let valid_digits: BTreeSet<Digit> = DIGITS.iter().cloned().collect();

    let mut acc = 0;
    'top: for (ws, ds) in wirings.iter().zip(&numbers) {
        // brute force go brrr
        for &k in &all_wirings {
            let rewired: BTreeSet<Digit> = ws.iter().map(|&w| w * k).collect();

            if rewired == valid_digits {
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
