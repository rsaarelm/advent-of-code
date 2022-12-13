use std::{cmp::Ordering, fmt, str::FromStr};

use aoc::prelude::*;
use itertools::Itertools;

#[derive(Clone)]
enum Item {
    Atom(u32),
    List(Vec<Item>),
}

use Item::*;

impl Eq for Item {}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        let (a, b) = (self.clone(), other.clone());
        let (a, b): (Item, Item) = match (a, b) {
            (Atom(a), Atom(b)) => return a.cmp(&b), // Early exit
            (a @ Atom(_), b) => (List(vec![a]), b),
            (a, b @ Atom(_)) => (a, List(vec![b])),
            c => c,
        };

        // They're both lists now.
        let (List(a), List(b)) = (a, b) else { panic!("Invalid items") };
        let (mut a, mut b) = (a.as_slice(), b.as_slice());

        loop {
            if a.is_empty() && b.is_empty() {
                return Ordering::Equal;
            } else if a.is_empty() {
                return Ordering::Less;
            } else if b.is_empty() {
                return Ordering::Greater;
            }

            let c = a[0].cmp(&b[0]);
            if c == Ordering::Equal {
                a = &a[1..];
                b = &b[1..];
                continue;
            } else {
                return c;
            }
        }
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
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
