use std::{collections::VecDeque, fmt};

use aoc::prelude::*;
use indexmap::IndexSet;

/// Static system configuration.
#[derive(Debug)]
struct Configuration {
    /// Module sorted in order broadcaster, flip-flops, conjunctors.
    connections: Vec<Vec<usize>>,
    /// Index of the first conjunctor module.
    conjunctors_start: usize,
    /// Inbound connection count for given module.
    n_inbound: Vec<usize>,
    /// A "rx" module is present and config is eligible for P2.
    has_rx: bool,
}

impl FromStr for Configuration {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut has_rx = false;

        let mut names = Vec::new();

        // Build mapping from names to indices.
        for line in s.lines() {
            let (module, _) = line.split_once(" -> ").unwrap();
            if module == "broadcaster" {
                names.push("$"); // Should sort before everything else.
            } else {
                names.push(module);
            }
        }

        names.sort();
        assert!(names[0] == "$");
        names[0] = "broadcaster";

        let conjunctors_start = names
            .iter()
            .position(|p| p.starts_with('&'))
            .unwrap_or(names.len());

        let mut connections = vec![Vec::default(); names.len()];
        let mut n_inbound = vec![0; names.len()];
        // Build connections.
        for line in s.lines() {
            let (module, outputs) = line.split_once(" -> ").unwrap();

            let outputs = outputs.split(", ").collect::<Vec<_>>();
            let i = names.iter().position(|&a| a == module).unwrap();

            for o in &outputs {
                if *o == "rx" {
                    has_rx = true;
                }

                let Some(j) = names.iter().position(|a| &&a[1..] == o) else {
                    // There might be dummy outputs that don't count.
                    continue;
                };

                n_inbound[j] += 1;
                connections[i].push(j);
            }
        }

        Ok(Configuration {
            connections,
            conjunctors_start,
            n_inbound,
            has_rx,
        })
    }
}

impl Configuration {
    fn successors(&self, n: usize) -> impl IntoIterator<Item = usize> {
        let mut result = IndexSet::new();
        let mut queue = VecDeque::from(self.connections[n].clone());
        while let Some(m) = queue.pop_front() {
            result.insert(m);

            for m in &self.connections[m] {
                if !result.contains(m) {
                    queue.push_back(*m);
                }
            }
        }

        result
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
enum Memory {
    Flip(bool),
    Conjunct(Vec<bool>),
}

impl fmt::Display for Memory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            &Flip(b) => write!(f, "{}", if b { '#' } else { '-' })?,
            Conjunct(bs) => {
                for &b in bs {
                    write!(f, "{}", if b { '#' } else { '-' })?;
                }
            }
        }

        Ok(())
    }
}

use Memory::*;

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct Network {
    state: Vec<Memory>,
}

impl fmt::Display for Network {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, s) in self.state.iter().enumerate() {
            write!(f, "{}{i}|{s}", if i > 0 { " " } else { "" })?;
        }

        Ok(())
    }
}

fn wires(is_high: bool) -> IVec2 {
    if is_high {
        ivec2(0, 1)
    } else {
        ivec2(1, 0)
    }
}

impl Network {
    fn new(config: &Configuration) -> Self {
        let n = config.connections.len();
        Network {
            state: (0..n)
                .map(|i| {
                    if i < config.conjunctors_start {
                        Flip(false)
                    } else {
                        Conjunct(vec![false; n])
                    }
                })
                .collect(),
        }
    }

    /// Return (low_count, high_count)
    fn cycle(
        &mut self,
        config: &Configuration,
        trap: &mut [bool],
    ) -> (IVec2, bool) {
        // (to, from, is_high?)
        let mut pending = VecDeque::from([(0, 0, false)]);
        let mut ret = ivec2(1, 0);
        let mut win = false;

        while let Some((module, from, is_high)) = pending.pop_front() {
            if module == 0 {
                // Broadcast
                assert!(!is_high);
                for &m in &config.connections[module] {
                    ret += ivec2(1, 0);
                    pending.push_back((m, 0, false));
                }
            } else if let Flip(ref mut b) = self.state[module] {
                // Flip-flop
                if !is_high {
                    *b = !*b;
                    let is_high = *b;

                    if !is_high && config.connections[module].is_empty() {
                        win = true;
                    }

                    ret += wires(is_high)
                        * config.connections[module].len().max(1) as i32;
                    for &m in &config.connections[module] {
                        pending.push_back((m, module, is_high));
                    }
                }
            } else if let Conjunct(ref mut ns) = self.state[module] {
                // Conjunctor
                ns[from] = is_high;
                let is_high = ns.iter().filter(|&&a| a).count()
                    != config.n_inbound[module];

                if !is_high {
                    trap[module] = true;
                }

                if !is_high && config.connections[module].is_empty() {
                    win = true;
                }

                ret += wires(is_high)
                    * config.connections[module].len().max(1) as i32;
                for &m in &config.connections[module] {
                    pending.push_back((m, module, is_high));
                }
            }
        }

        (ret, win)
    }
}

fn main() {
    let config: Configuration = stdin_string().parse().unwrap();

    let mut network = Network::new(&config);
    let mut trap = vec![false; config.connections.len()];
    let mut n = ivec2(0, 0);
    for _ in 0..1000 {
        n += network.cycle(&config, &mut trap).0;
    }
    println!("{}", n.x * n.y);

    // Skip P2 for examples.
    if !config.has_rx {
        println!("0");
        return;
    }

    // Each output from broadcaster corresponds to a "digit" counter. Locate
    // them and establish their periods.

    let mut digits = Vec::new();
    for &n in &config.connections[0] {
        digits.push(
            config
                .successors(n)
                .into_iter()
                .find(|&n| n >= config.conjunctors_start)
                .unwrap(),
        );
    }

    let mut periods = vec![0u64; digits.len()];

    let mut network = Network::new(&config);
    for i in 1.. {
        if periods.iter().all(|&a| a != 0) {
            break;
        }

        let mut trap = vec![false; config.connections.len()];
        network.cycle(&config, &mut trap);

        for (j, &d) in digits.iter().enumerate() {
            // The period is done when the first conjuctor found along
            // respective output from broadcaster emits a low signal.
            if trap[d] {
                periods[j] = i;
            }
        }
    }

    // The final signal goes out when all periods agree.

    println!("{}", periods.into_iter().reduce(num_integer::lcm).unwrap());
}
