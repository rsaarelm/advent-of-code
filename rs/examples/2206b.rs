use aoc::prelude::*;

struct Machine {
    counts: [usize; 256],
    total: usize,
}

impl Default for Machine {
    fn default() -> Self {
        Machine {
            counts: [0; 256],
            total: 0,
        }
    }
}

impl Machine {
    fn add(&mut self, n: u8) {
        let n = n as usize;
        if self.counts[n] == 0 {
            self.total += 1;
        }
        self.counts[n] += 1;
    }

    fn remove(&mut self, n: u8) {
        let n = n as usize;
        debug_assert!(self.counts[n] > 0);
        if self.counts[n] == 1 {
            self.total -= 1;
        }
        self.counts[n] -= 1;
    }

    fn run(&mut self, n: usize, input: &[u8]) -> usize {
        for i in 0..input.len() {
            self.add(input[i]);
            if i >= n {
                self.remove(input[i - n]);
            }
            if self.total == n {
                return i + 1;
            }
        }
        panic!("No marker found");
    }
}

fn main() {
    let input = stdin_string();

    println!("{}", Machine::default().run(4, input.as_bytes()));
    println!("{}", Machine::default().run(14, input.as_bytes()));
}
