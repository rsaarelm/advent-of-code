use derive_deref::{Deref, DerefMut};

use aoc::prelude::*;

#[derive(Clone, Debug, Eq, PartialEq, Hash, Deref, DerefMut)]
struct State(Vec<i32>);

impl FromStr for State {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn is_element(s: &str) -> bool {
            match s {
                "hydrogen" => true,
                x if x.ends_with("ium") => true,
                _ => false,
            }
        }

        let mut ret = vec![0];
        let mut elements = HashMap::default();

        // Build an [E, aG, aM, bG, bM, ...] state array.
        // Positions are protagonist's floor and successive generator and chip
        // pairs' floors.

        for (floor, line) in s.lines().enumerate() {
            let mut idx = 0;
            for word in line.split(|c: char| !c.is_alphabetic()) {
                if is_element(word) {
                    let default = elements.len() + 1;
                    idx = *elements.entry(word).or_insert(default);
                }
                let n = match word {
                    "generator" => idx * 2 - 1,
                    "microchip" => idx * 2,
                    _ => 0,
                };

                if n != 0 {
                    debug_assert!(idx > 0);
                    while ret.len() <= n {
                        ret.push(0);
                    }
                    ret[n] = floor as i32;
                    idx = 0;
                }
            }
        }

        Ok(State(ret))
    }
}

impl State {
    pub fn is_valid(&self) -> bool {
        if !self.iter().all(|x| (0..4).contains(x)) {
            return false;
        }

        for floor in 0..4 {
            let mut exposed_chips = false;
            let mut generators = false;
            for elt in (0..self.len()).skip(1).step_by(2) {
                if self[elt + 1] == floor && self[elt] != floor {
                    exposed_chips = true;
                }
                if self[elt] == floor {
                    generators = true;
                }
            }

            if exposed_chips && generators {
                return false;
            }
        }

        true
    }

    pub fn is_end(&self) -> bool {
        self.iter().all(|&x| x == 3)
    }

    pub fn neighbors(&self) -> Vec<State> {
        let mut ret = Vec::new();
        // Nonzero a and b correspond to the max two items you can carry
        // along.
        for a in 1..self.len() {
            for b in 0..a {
                // Are these items on the same floor with you?
                if self[a] != self[0] || self[b] != self[0] {
                    continue;
                }

                for f in [self[0] - 1, self[0] + 1] {
                    let mut s = self.clone();

                    s[0] = f;
                    s[a] = f;
                    s[b] = f;

                    if s.is_valid() {
                        ret.push(s);
                    }
                }
            }
        }

        ret
    }
}

fn main() {
    let p1: State = stdin_string().parse().unwrap();
    assert!(p1.is_valid());

    for (e, i) in dijkstra_map(State::neighbors, &p1) {
        if e.is_end() {
            println!("{i}");
            break;
        }
    }

    let mut p2 = p1.clone();
    p2.extend([0, 0, 0, 0]);

    for (e, i) in dijkstra_map(State::neighbors, &p2) {
        if e.is_end() {
            println!("{i}");
            break;
        }
    }
}
