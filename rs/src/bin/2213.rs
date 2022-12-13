use std::{cmp::Ordering, fmt, str::FromStr};

use aoc::prelude::*;
use itertools::Itertools;

#[derive(Clone)]
enum Item {
    Atom(u32),
    List(Vec<Item>),
}

use Item::*;

fn main() {
    let mut packets = vec![Vec::new()];
    for item in stdin_lines() {
        if item.trim().is_empty() {
            packets.push(Vec::new());
        } else {
            let item = item.parse::<Item>().unwrap();
            let i = packets.len() - 1;
            packets[i].push(item);
        }
    }

    // Part 1

    let mut sorted = 0;
    for (i, p) in packets.iter().enumerate() {
        // XXX: Complex way to check if sequence is sorted since is_sorted()
        // method is still unstable.
        if p.iter()
            .try_fold(&p[0], |a, b| (a <= b).then_some(b).ok_or(()))
            .is_ok()
        {
            // Indexing starts from 1.
            sorted += i + 1;
        }
    }

    println!("{}", sorted);

    // Part 2

    let p2: Item = "[[2]]".parse().unwrap();
    let p6: Item = "[[6]]".parse().unwrap();

    let mut list = vec![p2.clone(), p6.clone()];

    for mut p in packets {
        list.append(&mut p);
    }

    list.sort();

    println!(
        "{}",
        [p2, p6]
            .iter()
            .map(|p| list.iter().position(|x| x == p).unwrap() + 1)
            .product::<usize>()
    );
}

impl FromStr for Item {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse(mut s: &str) -> Result<(Item, &str), <Item as FromStr>::Err> {
            // Recursive descent parser.
            if s.is_empty() {
                return Err(());
            }

            // Ignore commas.
            while s.starts_with(',') {
                s = &s[1..];
            }

            let c = s.chars().next().unwrap();
            if c == '[' {
                let mut s = &s[1..]; // Drop the opening '['.
                let mut ret: Vec<Item> = Vec::new();
                // Parse items from list.
                while !s.is_empty() && !s.starts_with(']') {
                    let (item, rest) = parse(s)?;
                    ret.push(item);
                    s = rest;
                }
                // Drop the closing ].
                if s.starts_with(']') {
                    s = &s[1..];
                }

                Ok((Item::List(ret), s))
            } else if c.is_ascii_digit() {
                let number: String = s.chars().take_while(|c| c.is_ascii_digit()).collect();
                Ok((Item::Atom(number.parse().unwrap()), &s[number.len()..]))
            } else {
                Err(())
            }
        }

        parse(s).map(|(item, _)| item)
    }
}

// For completeness' sake
impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Atom(n) => write!(f, "{}", n),
            List(a) => {
                write!(f, "[")?;
                write!(f, "{}", a.iter().format(","))?;
                write!(f, "]")
            }
        }
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        let mut atom_list = Vec::new();

        // Convert to slices, using atom_list as scratch space for atomic
        // items.
        let (a, b): (&[Item], &[Item]) = match (self, other) {
            (Atom(a), Atom(b)) => return a.cmp(b), // Early exit
            (a @ Atom(_), List(b)) => {
                atom_list.push(a.clone());
                (atom_list.as_slice(), b.as_slice())
            }
            (List(a), b @ Atom(_)) => {
                atom_list.push(b.clone());
                (a.as_slice(), atom_list.as_slice())
            }
            (List(a), List(b)) => (a.as_slice(), b.as_slice()),
        };

        // Built-in lexical comparison for slices can take it from here.
        a.cmp(b)
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Item {}
