use rand::prelude::*;

use aoc::prelude::*;

#[derive(Copy, Clone, Debug)]
enum Cell {
    And(isize, isize),
    Or(isize, isize),
    Xor(isize, isize),
}

impl Cell {
    fn ops(self) -> (isize, isize) {
        match self {
            And(a, b) | Or(a, b) | Xor(a, b) => (a, b),
        }
    }
}

use Cell::*;

#[derive(Clone, Debug)]
struct Alu {
    mem: Vec<Cell>,
    names: Vec<String>,
    z_start: usize,
}

impl Alu {
    fn eval(&self, n: isize, x: u64, y: u64) -> u64 {
        if n < -64 {
            ((y & 1 << (-n - 65)) != 0) as u64
        } else if n < 0 {
            ((x & 1 << (-n - 1)) != 0) as u64
        } else {
            match self.mem[n as usize] {
                And(a, b) => self.eval(a, x, y) & self.eval(b, x, y),
                Or(a, b) => self.eval(a, x, y) | self.eval(b, x, y),
                Xor(a, b) => self.eval(a, x, y) ^ self.eval(b, x, y),
            }
        }
    }

    pub fn run(&self, x: u64, y: u64) -> u64 {
        let mut ret = 0;
        for i in self.z_start..self.mem.len() {
            ret |= self.eval(i as isize, x, y) << (i - self.z_start);
        }
        ret
    }

    pub fn error(&self, n: usize) -> f32 {
        // Cycle detection.
        let mut graph = petgraph::Graph::<(), ()>::new();
        let nodes = self
            .mem
            .iter()
            .map(|_| graph.add_node(()))
            .collect::<Vec<_>>();
        for (i, m) in self.mem.iter().enumerate() {
            let (a, b) = m.ops();
            if a >= 0 {
                graph.add_edge(nodes[i], nodes[a as usize], ());
            }
            if b >= 0 {
                graph.add_edge(nodes[i], nodes[b as usize], ());
            }
        }

        // Invalid circuit, return maximum error.
        if petgraph::algo::is_cyclic_directed(&graph) {
            return 1.0;
        }

        let bits = self.mem.len() - self.z_start - 1;
        let mask = (1 << bits) - 1;
        let mut errs = Vec::new();
        let mut rng = thread_rng();

        for _ in 0..n {
            let (x, y) = (rng.gen::<u64>() & mask, rng.gen::<u64>() & mask);
            let z = self.run(x, y);
            let mut bad = 0;
            for i in 0..64 {
                if z & (1 << i) != (x + y) & (1 << i) {
                    bad += 1;
                }
            }
            errs.push(bad as f32 / 64.0);
        }
        errs.iter().sum::<f32>() / (errs.len() as f32)
    }
}

struct Input(u64, u64, Alu);

impl FromStr for Input {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn idx(elts: &[String], a: &str) -> isize {
            if let Some(num) = a.strip_prefix("x") {
                -1 - num.parse::<isize>().unwrap()
            } else if let Some(num) = a.strip_prefix("y") {
                -65 - num.parse::<isize>().unwrap()
            } else {
                elts.iter().position(|e| e == a).unwrap() as isize
            }
        }

        let mut names: HashSet<&str> = HashSet::default();
        let (mut x, mut y) = (0, 0);

        // Collect element lists.
        for line in s.lines() {
            let parts = line.split(' ').collect::<Vec<_>>();
            if parts.len() == 5 {
                names.insert(parts[0]);
                names.insert(parts[2]);
                names.insert(parts[4]);
            }
        }

        let mut names = names
            .into_iter()
            .filter(|e| !e.starts_with('x') && !e.starts_with('y'))
            .map(|s| s.to_owned())
            .collect::<Vec<_>>();
        names.sort();
        let z_start = names.iter().position(|a| a.starts_with('z')).unwrap();

        let mut mem = vec![And(0, 0); names.len()];

        // Actual parse now that we can assign memory indices.
        for line in s.lines() {
            // Register data.
            if let Some((a, b)) = line.split_once(": ") {
                let b: u64 = b.parse().unwrap();
                if let Some(n) = a.strip_prefix("x") {
                    let n: u64 = n.parse().unwrap();
                    x |= b << n;
                } else if let Some(n) = a.strip_prefix("y") {
                    let n: u64 = n.parse().unwrap();
                    y |= b << n;
                } else {
                    return Err(());
                }
                continue;
            }

            // ALU data.
            let parts = line.split(' ').collect::<Vec<_>>();
            if parts.len() != 5 {
                continue;
            }
            let a = idx(&names, parts[0]);
            let b = idx(&names, parts[2]);
            let i = names.iter().position(|a| a == parts[4]).unwrap();

            mem[i] = match parts[1] {
                "XOR" => Xor(a, b),
                "OR" => Or(a, b),
                "AND" => And(a, b),
                _ => return Err(()),
            }
        }

        Ok(Input(
            x,
            y,
            Alu {
                mem,
                names,
                z_start,
            },
        ))
    }
}

fn main() {
    let Input(x, y, alu) = from_stdin();

    println!("{}", alu.run(x, y));

    // Examples won't do P2.
    if alu.mem.len() < 50 {
        eprintln!("Example input detected, bailing out of P2");
        return;
    }

    let err_baseline = alu.error(1000);

    // Since scoring is random, things are a bit wobbly, try a few times to
    // hit the solution.
    for _retry in 0..10 {
        let mut swap_scores = HashMap::default();
        for i in 0..alu.mem.len() {
            for j in (i + 1)..alu.mem.len() {
                if j % 11 == 0 {
                    eprint!(
                        "Evaluating swaps: {} / {}            \r",
                        j + i * alu.mem.len(),
                        alu.mem.len() * alu.mem.len()
                    );
                }

                let mut blu = alu.clone();
                blu.mem.swap(i, j);

                // Get a quick look at error.
                let mut error = blu.error(10);
                if error < err_baseline * 0.9 {
                    // It looks small, get a more precise value.
                    error = blu.error(1000);
                }

                swap_scores.insert((i, j), error);
            }
        }

        // Figure out pairs that improve the system the most. Make sure
        // solution consists of unique swaps, the same node may show up
        // multiple times in top swaps.
        let mut pairs = swap_scores.keys().cloned().collect::<Vec<_>>();
        pairs.sort_by_key(|a| (swap_scores[a] * 100000.0) as i64);
        let mut sln = Vec::new();
        for (x, y) in &pairs {
            if sln.len() == 4 {
                break;
            }
            if sln
                .iter()
                .any(|(i, j)| i == x || i == y || j == x || j == y)
            {
                continue;
            }

            sln.push((*x, *y));
        }

        let mut blu = alu.clone();
        eprint!("Verifying answer, swapping");
        for &(i, j) in &sln {
            eprint!(" {}-{}", alu.names[i], alu.names[j]);
            blu.mem.swap(i, j);
        }
        eprintln!();
        if blu.error(100) > 0.0 {
            eprintln!("Didn't find answer, retrying...");
            continue;
        }

        let mut elts = pairs
            .into_iter()
            .take(4)
            .flat_map(|(a, b)| [alu.names[a].as_ref(), alu.names[b].as_ref()])
            .collect::<Vec<_>>();
        elts.sort();

        println!("{}", elts.join(","));

        return;
    }

    eprintln!("Failed to solve P2, sorry.");
}
