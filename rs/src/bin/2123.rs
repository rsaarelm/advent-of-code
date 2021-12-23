use aoc::prelude::*;
use std::{
    collections::{BTreeMap, BinaryHeap},
    fmt,
};

// ##############################################
// ## 8   9       10     11     12      13  14 ##
// ##########  1  ##  3  ##  5  ##  7  ##########
//         ##  0  ##  2  ##  4  ##  6  ##
//         ##############################

fn is_corridor(p: usize) -> bool {
    p >= 8
}

#[rustfmt::skip]
/// Coordinate positions of occupancy slots.
const XY: [[i32; 2]; 15] = [
    [2, 2], [2, 1],  [4, 2], [4, 1],  [6, 2], [6, 1],  [8, 2], [8, 1],
    [0, 0], [1, 0], [3, 0], [5, 0], [7, 0], [9, 0], [10, 0],
];

const COST: [u64; 5] = [0, 1, 10, 100, 1000];

/// Manhattan distance between two slots.
fn dist(p1: usize, p2: usize) -> u64 {
    if p1 < 8 && p2 < 8 {
        debug_assert!(p1 / 2 != p2 / 2, "No jumping in burrows");
        // Bin to bin
        (XY[p1][1] + XY[p2][1] + (XY[p2][0] - XY[p1][0]).abs()) as u64
    } else {
        debug_assert!(
            !is_corridor(p1) || !is_corridor(p2),
            "No running in corridors"
        );
        // Between bin and corridor
        ((XY[p2][1] - XY[p1][1]).abs() + (XY[p2][0] - XY[p1][0]).abs()) as u64
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
struct State(u64);

impl State {
    pub fn new(input: &Vec<char>) -> State {
        let mut ret = State(0);
        // Slot positions in the order they show up in input.
        for (&c, i) in input.iter().zip([1, 3, 5, 7, 0, 2, 4, 6]) {
            ret.set(i, c as u8 - 'A' as u8 + 1);
        }
        ret
    }

    // Can't use IndexMut to access bit slices, gotta write things ugly-like.
    pub fn get(self, idx: usize) -> u8 {
        debug_assert!(idx <= 15);
        ((self.0 >> (idx * 3)) & 0b111) as u8
    }

    pub fn set(&mut self, idx: usize, n: u8) {
        debug_assert!(idx <= 15);
        debug_assert!(n <= 4);
        let n = (n as u64 & 0b111) << (idx * 3);
        let mask = 0b111 << (idx * 3);
        *self = State(self.0 & (!mask) | n);
    }

    /// Return whether there's no obstruction between points.
    ///
    /// Origin point may be occupied.
    pub fn can_move(self, p1: usize, p2: usize) -> bool {
        if p1 == p2 {
            return false;
        }

        if self.get(p2) != 0 {
            // Destination is occupied.
            return false;
        }

        if is_corridor(p1) && is_corridor(p2) {
            // At least one point must be in burrow.
            return false;
        }

        if p1 < 8 && p1 / 2 == p2 / 2 {
            // Must leave burrow.
            return false;
        }

        if p1 < 8 && (p1 % 2) == 0 && self.get(p1 + 1) != 0 {
            // Can't move out of burrow past another.
            return false;
        }

        if p2 < 8 && (p2 % 2) == 1 && self.get(p2 - 1) == 0 {
            // Can't enter top slot of burrow unless bottom slot is occupied.
            return false;
        }

        let (x1, x2) = (XY[p1][0], XY[p2][0]);
        debug_assert!(x1 != x2);
        let (x1, x2) = (x1.min(x2), x1.max(x2));

        for i in 8..15 {
            if XY[i][0] > x1 && XY[i][0] < x2 && self.get(i) != 0 {
                // Corridor is blocked.
                return false;
            }
        }

        true
    }

    /// Count how far from solution this state is.
    ///
    /// Estimate energy cost of movement needed to fix the state to compute
    /// badness. Result should be somewhere around the lower bound of actual
    /// cost needed to move to finish state.
    pub fn badness(self) -> u64 {
        // Top home positions for different kinds.
        const HOME: [usize; 5] = [0, 1, 3, 5, 7];
        const COST2: [u64; 5] = [0, 1, 2, 3, 4];

        let mut ret = 0;

        for i in 0..15 {
            if self.get(i) == 0 {
                // Ignore empty positions.
                continue;
            }

            let home_type = (if i < 8 { i / 2 + 1 } else { 0 }) as u8;

            if self.get(i) == home_type {
                if i < 8 && i % 2 == 1 && self.get(i - 1) != 0 && self.get(i - 1) != home_type {
                    // At top of burrow and we're blocking a non-belonging
                    // element, add the minimum cost of moving out of the
                    // way and back in.
                    ret += COST2[self.get(i) as usize] * 4;
                }

                continue;
            } else {
                // Misplaced element, add cost of straightest path to top of
                // home burrow.
                ret += dist(HOME[self.get(i) as usize], i) * COST2[self.get(i) as usize];
            }
        }

        return ret;
    }

    pub fn is_clean_home(self, kind: u8) -> bool {
        debug_assert!(kind > 0);
        let idx = (kind as usize - 1) * 2;
        // Contents in this kind's bin.
        let (a, b) = (self.get(idx), self.get(idx + 1));
        // Must be empty or containing only the correct inhabitants.
        (a == 0 || a == kind) && (b == 0 || b == kind)
    }

    /// Cell has occupant in correct burrow who will not move again.
    pub fn is_settled(self, p: usize) -> bool {
        let kind = (p / 2) as u8 + 1;
        p < 8 && self.is_clean_home(kind) && self.get(p) == kind
    }

    /// Cost for path for the amphipod at start of path.
    pub fn cost(self, p1: usize, p2: usize) -> u64 {
        debug_assert!(self.get(p1) != 0);
        let dist = dist(p1, p2) as u64;
        let cost = COST[self.get(p1) as usize];
        dist * cost
    }

    /// Enumerate transitions from current state with energy expenditures.
    pub fn transitions(self) -> Vec<(u64, State)> {
        let mut ret = Vec::new();
        for start in 0..15 {
            if self.get(start) == 0 || self.is_settled(start) {
                // No mover here.
                continue;
            }
            for end in 0..15 {
                if !self.can_move(start, end) {
                    continue;
                }
                let mut new_state = self;
                new_state.set(start, 0);
                new_state.set(end, self.get(start));
                ret.push((self.cost(start, end), new_state));
            }
        }

        return ret;
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = |i| b".ABCD"[self.get(i) as usize] as char;

        writeln!(f, "#############")?;

        write!(f, "#")?;
        write!(f, "{}", c(8))?;
        write!(f, "{}", c(9))?;
        write!(f, ".")?;
        write!(f, "{}", c(10))?;
        write!(f, ".")?;
        write!(f, "{}", c(11))?;
        write!(f, ".")?;
        write!(f, "{}", c(12))?;
        write!(f, ".")?;
        write!(f, "{}", c(13))?;
        write!(f, "{}", c(14))?;
        writeln!(f, "#")?;

        write!(f, "###")?;
        write!(f, "{}", c(1))?;
        write!(f, "#")?;
        write!(f, "{}", c(3))?;
        write!(f, "#")?;
        write!(f, "{}", c(5))?;
        write!(f, "#")?;
        write!(f, "{}", c(7))?;
        writeln!(f, "###")?;

        write!(f, "  #")?;
        write!(f, "{}", c(0))?;
        write!(f, "#")?;
        write!(f, "{}", c(2))?;
        write!(f, "#")?;
        write!(f, "{}", c(4))?;
        write!(f, "#")?;
        write!(f, "{}", c(6))?;
        writeln!(f, "#")?;

        writeln!(f, "  #########")
    }
}

fn main() {
    let state = State::new(
        &stdin_string()
            .chars()
            .filter(|c| c.is_ascii_alphabetic())
            .collect::<Vec<char>>(),
    );

    let mut open = BinaryHeap::new();
    let mut cost = BTreeMap::new();
    cost.insert(state, 0);
    open.push((state.badness(), state));

    while let Some((badness, s)) = open.pop() {
        if badness == 0 {
            println!("{}", cost[&s]);
            break;
        }

        let initial_cost = cost[&s];

        for (added_cost, s) in s.transitions().into_iter() {
            if cost.contains_key(&s) && cost[&s] <= initial_cost + added_cost {
                continue;
            }
            eprint!("{}   \r", s.badness());

            cost.insert(s, initial_cost + added_cost);
            open.push((s.badness(), s));
        }
    }
}
