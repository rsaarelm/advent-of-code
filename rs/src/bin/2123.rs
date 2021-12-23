#![feature(let_else)]
use aoc::prelude::*;
use std::{
    collections::{BTreeMap, BinaryHeap},
    fmt, iter,
};

//  ##############################################
//  # 16  17      18      19      20      21  22 #
//  #########  3 ####  7 #### 11 #### 15 #########
//          #  2 #  #  6 #  # 10 #  # 14 #
//          #  1 #  #  5 #  #  9 #  # 13 #
//          #  0 #  #  4 #  #  8 #  # 12 #
//          ######  ######  ######  ######
//
//                   Array indices

//  ##############################################
//  # 00  10  20  30  40  50  60  70  80  90  A0 #
//  ######### 21 #### 41 #### 61 #### 81 #########
//          # 22 #  # 42 #  # 62 #  # 82 #
//          # 23 #  # 43 #  # 63 #  # 83 #
//          # 24 #  # 44 #  # 64 #  # 84 #
//          ######  ######  ######  ######
//
//                  XY coordinates

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
enum Amphipod {
    Amber = 0,
    Bronze = 1,
    Copper = 2,
    Desert = 3,

    // Blocks bottom of burrow in part 1, does not move.
    Rubble = 4,
}

use Amphipod::*;

impl From<char> for Amphipod {
    fn from(c: char) -> Self {
        match c {
            'A' => Amber,
            'B' => Bronze,
            'C' => Copper,
            'D' => Desert,
            _ => Rubble,
        }
    }
}

impl Into<char> for Amphipod {
    fn into(self) -> char {
        match self {
            Amber => 'A',
            Bronze => 'B',
            Copper => 'C',
            Desert => 'D',
            Rubble => '#',
        }
    }
}

impl Amphipod {
    fn cost(self) -> u64 {
        match self {
            Amber => 1,
            Bronze => 10,
            Copper => 100,
            Desert => 1000,
            Rubble => panic!("rubble rubble"),
        }
    }

    /// Badness estimation target cell.
    fn home_cell(self) -> usize {
        match self {
            Amber => 3,
            Bronze => 7,
            Copper => 11,
            Desert => 15,
            Rubble => panic!("rubble rubble"),
        }
    }

    fn home_cells(self) -> std::ops::Range<usize> {
        assert!(self != Rubble);
        let base = 4 * self as usize;
        base..base + 4
    }
}

fn burrow(p: usize) -> Option<usize> {
    if p < 16 {
        Some(p / 4)
    } else {
        None
    }
}

/// Map x position to bin.
fn bin(x: i32) -> Option<Amphipod> {
    match x {
        2 => Some(Amber),
        4 => Some(Bronze),
        6 => Some(Copper),
        8 => Some(Desert),
        _ => None,
    }
}

const N_CELLS: usize = 23;

#[rustfmt::skip]
/// Coordinate positions of occupancy slots.
const XY: [[i32; 2]; N_CELLS] = [
    [2, 4], [2, 3], [2, 2], [2, 1],
    [4, 4], [4, 3], [4, 2], [4, 1],
    [6, 4], [6, 3], [6, 2], [6, 1],
    [8, 4], [8, 3], [8, 2], [8, 1],

    [0, 0], [1, 0], [3, 0], [5, 0], [7, 0], [9, 0], [10, 0],
];

const COST: [u64; 4] = [1, 10, 100, 1000];

/// Manhattan distance between two slots.
fn dist(p1: usize, p2: usize) -> u64 {
    if burrow(p1).is_some() && burrow(p2).is_some() {
        debug_assert!(burrow(p1) != burrow(p2), "No jumping in burrows");
        // Bin to bin
        (XY[p1][1] + XY[p2][1] + (XY[p2][0] - XY[p1][0]).abs()) as u64
    } else {
        debug_assert!(
            burrow(p1).is_some() || burrow(p2).is_some(),
            "No running in corridors"
        );
        // Between bin and corridor
        ((XY[p2][1] - XY[p1][1]).abs() + (XY[p2][0] - XY[p1][0]).abs()) as u64
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
struct State([Option<Amphipod>; N_CELLS]);

impl std::ops::Index<usize> for State {
    type Output = Option<Amphipod>;

    fn index(&self, index: usize) -> &Self::Output {
        debug_assert!(index < N_CELLS);
        &self.0[index]
    }
}

impl std::ops::IndexMut<usize> for State {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        debug_assert!(index < N_CELLS);
        &mut self.0[index]
    }
}

impl State {
    pub fn new(input: &Vec<char>) -> State {
        let mut ret = State(Default::default());
        // Slot positions in the order they show up in input.
        // Fill in the rest with the dummy 0xf value if input only has the
        // first two lines.
        for (c, i) in input
            .iter()
            .cloned()
            .chain(iter::repeat('E'))
            .zip([3, 7, 11, 15, 2, 6, 10, 14, 1, 5, 9, 13, 0, 4, 8, 12])
        {
            ret[i] = Some(Amphipod::from(c));
        }
        ret
    }

    /// Count how far from solution this state is.
    ///
    /// Estimate energy cost of movement needed to fix the state to compute
    /// badness. Result should be somewhere around the lower bound of actual
    /// cost needed to move to finish state.
    pub fn badness(self) -> u64 {
        let mut ret = 0;

        for i in 0..N_CELLS {
            let Some(a) = self[i] else { continue };
            if a == Rubble {
                continue;
            }

            let [x, y] = XY[i];

            if let Some(bin) = bin(x) {
                debug_assert!(y > 0);

                if bin != a {
                    // In a foreign bin, trace distance home.
                    // Double the badness to encourage getting amphipods out
                    // of the wrong bins.
                    ret += dist(i, a.home_cell()) * COST[a as usize] * 2;
                } else {
                    // Are there mismatched amphipods trapped below this one?
                    let base = (i / 4) * 4;
                    for j in base..i {
                        let b = self[j].expect("Corrupt state, empty space below amphipod");
                        if b != Rubble && b != a {
                            // Self must move to unblock, add an arbitrary
                            // helping of badness.
                            ret += 10 * COST[a as usize];
                            continue;
                        }
                    }
                }
            } else {
                // In corridor, trace distance home.
                ret += dist(i, a.home_cell()) * COST[a as usize];
            }
        }

        return ret;
    }

    /// If move between two x positions is possible, return updated state and
    /// energy cost of move.
    pub fn try_move(self, x1: i32, x2: i32) -> Option<(u64, State)> {
        if bin(x1).is_none() && bin(x2).is_none() {
            // No running in corridors.
            return None;
        }

        // Starting point.
        let p1;
        if let Some(bin) = bin(x1) {
            // Moving from bin. Find top item to move.
            let Some(p) = bin.home_cells().rev().find(|&i|
                self[i].is_some()) else {
                return None;
            };
            if self[p] == Some(Rubble) {
                // Don't move rubble.
                return None;
            }

            let base = 4 * bin as usize;
            if self[p] == Some(bin) {
                // Amphipod is in home bin, only allow moving it if it's
                // blocking a mismatching amphipod below it.
                if (base..p).all(|i| self[i] == Some(Rubble) || self[i] == Some(bin)) {
                    // Everything below is good, abort move.
                    return None;
                }
            }

            // Valid start pos established.
            p1 = p;
        } else {
            // Moving from corridor.
            p1 = XY
                .iter()
                .position(|[x, _]| *x == x1)
                .expect("Corridor cell with x pos not found");
        }
        // Now we see what we're moving.
        let Some(amphipod) = self[p1] else { return None; };
        debug_assert!(amphipod != Rubble);

        // Destination point.
        let p2;
        if let Some(bin) = bin(x2) {
            if bin != amphipod {
                // Not going home.
                return None;
            }

            // Find first empty slot in bin.
            let Some(p) = bin.home_cells().find(|&i|
                self[i].is_none()) else {
                // Fail if bin is full.
                return None;
            };

            if bin
                .home_cells()
                .any(|p| matches!(self[p], Some(a) if a != bin && a != Rubble))
            {
                // Mismatched amphipods in bin, can't enter yet.
                return None;
            }

            p2 = p;
        } else {
            // Moving to corridor.
            p2 = XY
                .iter()
                .position(|[x, _]| *x == x2)
                .expect("Corridor cell with x pos not found");
        }

        // Check in-between corridor slots for obstacles.
        let left_x = XY[p1][0].min(XY[p2][0]);
        let right_x = XY[p1][0].max(XY[p2][0]);
        for corr in 16..=22 {
            let cx = XY[corr][0];
            if corr != p1 && cx >= left_x && cx <= right_x && self[corr].is_some() {
                // Something on the way (that isn't us at p1), no go.
                return None;
            }
        }

        // Everything's good, perform the move!
        let mut new_state = self;
        new_state[p1] = None;
        new_state[p2] = self[p1];
        let cost = dist(p1, p2) * self[p1].unwrap().cost();

        Some((cost, new_state))
    }

    /// Enumerate transitions from current state with energy expenditures.
    pub fn transitions(self) -> Vec<(u64, State)> {
        let mut ret = Vec::new();
        for i in 0..=10 {
            for j in 0..=10 {
                if let Some(p) = self.try_move(i, j) {
                    ret.push(p)
                }
            }
        }
        ret
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = |i| self[i].map(|a| a.into()).unwrap_or('.');

        writeln!(f, "#############")?;

        write!(f, "#")?;
        write!(f, "{}", c(16))?;
        write!(f, "{}", c(17))?;
        write!(f, ".")?;
        write!(f, "{}", c(18))?;
        write!(f, ".")?;
        write!(f, "{}", c(19))?;
        write!(f, ".")?;
        write!(f, "{}", c(20))?;
        write!(f, ".")?;
        write!(f, "{}", c(21))?;
        write!(f, "{}", c(22))?;
        writeln!(f, "#")?;

        for y in 0..4 {
            if y == 0 {
                write!(f, "###")?;
            } else {
                write!(f, "  #")?;
            }
            write!(f, "{}", c(3 - y))?;
            write!(f, "#")?;
            write!(f, "{}", c(7 - y))?;
            write!(f, "#")?;
            write!(f, "{}", c(11 - y))?;
            write!(f, "#")?;
            write!(f, "{}", c(15 - y))?;
            if y == 0 {
                writeln!(f, "###")?;
            } else {
                writeln!(f, "#")?;
            }
        }
        writeln!(f, "  #########")
    }
}

fn solve(state: State) {
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

            cost.insert(s, initial_cost + added_cost);
            open.push((s.badness(), s));
        }
    }
}

fn main() {
    let input: Vec<char> = stdin_string()
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .collect();

    let mut input2 = input.clone();
    for c in "DCBADBAC".chars().rev() {
        input2.insert(4, c);
    }

    solve(State::new(&input));
    solve(State::new(&input2));
}
