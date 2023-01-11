use std::fmt;

use aoc::prelude::*;

#[derive(Copy, Clone, Default, Eq, PartialEq, Debug)]
struct S(pub i64);

impl FromStr for S {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut acc = 0;
        for (n, c) in s.chars().rev().enumerate() {
            let n = 5_i64.pow(n as u32);
            let c = "=-012".find(c).ok_or(())? as i64 - 2;
            acc += n * c;
        }
        Ok(S(acc))
    }
}

impl fmt::Display for S {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut acc = String::new();
        let mut x = self.0;
        while x != 0 {
            acc.push("012=-".as_bytes()[x.rem_euclid(5) as usize] as char);
            x = (x + 2) / 5;
        }
        acc = acc.chars().rev().collect();
        write!(f, "{acc}")
    }
}

fn main() {
    println!("{}", stdin_lines_as::<S>().fold(S(0), |a, b| S(a.0 + b.0)));
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
            assert_eq!(S(n).to_string(), s);
            assert_eq!(s.parse::<S>().unwrap().0, n);
        }
    }
}
