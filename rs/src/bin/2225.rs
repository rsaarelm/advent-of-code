use core::fmt;
use std::str::FromStr;

use aoc::prelude::*;

#[derive(Copy, Clone, Default, Eq, PartialEq, Debug)]
struct Snafu(pub i64);

impl FromStr for Snafu {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut acc = 0;
        for (n, c) in s.chars().rev().enumerate() {
            let n = n as u32;
            let a = match c {
                '0' => 0,
                '1' => 1,
                '2' => 2,
                '-' => -1,
                '=' => -2,
                _ => return Err(()),
            };
            acc += 5_i64.pow(n) * a;
        }
        Ok(Snafu(acc))
    }
}

impl fmt::Display for Snafu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut acc = String::new();
        let mut x = self.0;
        for _ in 0.. {
            let c = match x.rem_euclid(5) {
                0 => '0',
                1 => '1',
                2 => '2',
                3 => '=',
                4 => '-',
                _ => unreachable!(),
            };
            acc.push(c);

            x += 2;
            x /= 5;
            if x == 0 {
                break;
            }
        }
        acc = acc.chars().rev().collect();
        write!(f, "{acc}")
    }
}

fn main() {
    println!(
        "{}",
        stdin_lines_as::<Snafu>().fold(Snafu(0), |a, b| Snafu(a.0 + b.0))
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snafus() {
        for (n, s) in [
            (1, "1"),
            (2, "2"),
            (3, "1="),
            (4, "1-"),
            (5, "10"),
            (6, "11"),
            (7, "12"),
            (8, "2="),
            (9, "2-"),
            (10, "20"),
            (15, "1=0"),
            (20, "1-0"),
            (2022, "1=11-2"),
            (12345, "1-0---0"),
            (314159265, "1121-1110-1=0"),
        ] {
            assert_eq!(Snafu(n).to_string(), s);
            assert_eq!(s.parse::<Snafu>().unwrap().0, n);
        }
    }
}
