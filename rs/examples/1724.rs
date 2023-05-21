use std::{cmp::Ordering, collections::BinaryHeap};

use aoc::prelude::*;

type Pair = [u32; 2];

#[derive(Clone, Eq, PartialEq, Debug)]
struct State {
    path: Vec<Pair>,
    connected: usize,
    value: u32,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl State {
    fn initials(input: &[Pair]) -> Vec<State> {
        let mut ret = Vec::new();
        for (i, [a, b]) in input.iter().enumerate() {
            if *a == 0 {
                let mut path = input.to_vec();
                path.swap(0, i);
                ret.push(State {
                    path,
                    connected: 1,
                    value: *b,
                });
            }
        }
        ret
    }

    fn evolve(&self) -> Vec<State> {
        let mut ret = Vec::new();
        for i in self.connected..(self.path.len()) {
            // The unconnected terminal.
            let c = self.path[self.connected - 1][1];
            let [a, b] = self.path[i];
            if a == c || b == c {
                let mut path = self.path.clone();
                path.swap(i, self.connected);
                // Flip the port if needed, the unused terminal must always go
                // right.
                if b == c {
                    (path[self.connected][0], path[self.connected][1]) =
                        (path[self.connected][1], path[self.connected][0]);
                }
                let value =
                    self.value + path[self.connected].iter().sum::<u32>();
                ret.push(State {
                    path,
                    connected: self.connected + 1,
                    value,
                });
            }
        }

        ret
    }
}

fn main() {
    let input: Vec<Pair> = stdin_lines().map(fixed_numbers).collect();

    let mut heap: BinaryHeap<State> =
        State::initials(&input).into_iter().collect();

    let mut strongest = 0;
    let mut max_len = 0;
    let mut strongest_longest = 0;
    while let Some(top) = heap.pop() {
        for e in top.evolve() {
            strongest = strongest.max(e.value);
            match e.connected {
                a if a > max_len => {
                    max_len = a;
                    strongest_longest = e.value;
                }
                a if a == max_len => {
                    strongest_longest = strongest_longest.max(e.value);
                }
                _ => {}
            }
            heap.push(e);
        }
    }

    println!("{strongest}");
    println!("{strongest_longest}");
}
