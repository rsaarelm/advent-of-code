use std::cmp::Ordering;

use aoc::prelude::*;
use serde::Deserialize;

#[derive(Clone, Deserialize)]
#[serde(untagged)]
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
            let item = serde_json::from_str(&item).unwrap();
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

    let p2: Item = serde_json::from_str(&"[[2]]").unwrap();
    let p6: Item = serde_json::from_str(&"[[6]]").unwrap();

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
