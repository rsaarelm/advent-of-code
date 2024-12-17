use aoc::prelude::*;

const ADV: u64 = 0;
const BXL: u64 = 1;
const BST: u64 = 2;
const JNZ: u64 = 3;
const BCX: u64 = 4;
const OUT: u64 = 5;
const BDV: u64 = 6;
const CDV: u64 = 7;

#[derive(Clone, Debug)]
struct Machine {
    a: u64,
    b: u64,
    c: u64,
    pc: usize,
    prog: Vec<u64>,
    pub stdout: Vec<u64>,
}

impl Machine {
    fn combo(&self, op: u64) -> u64 {
        match op {
            4 => self.a,
            5 => self.b,
            6 => self.c,
            7 => panic!(),
            x => x,
        }
    }

    fn step(&mut self) -> bool {
        if self.pc >= self.prog.len() {
            return false;
        }

        let arg = self.prog[self.pc + 1];
        match self.prog[self.pc] {
            ADV => {
                //self.a /= 1 << self.combo(arg);
                self.a >>= self.combo(arg);
                self.pc += 2;
            }
            BXL => {
                self.b ^= arg;
                self.pc += 2;
            }
            BST => {
                self.b = self.combo(arg) & 7;
                self.pc += 2;
            }
            JNZ => {
                if self.a != 0 {
                    self.pc = arg as usize;
                } else {
                    self.pc += 2;
                }
            }
            BCX => {
                self.b ^= self.c;
                self.pc += 2;
            }
            OUT => {
                self.stdout.push(self.combo(arg) & 7);
                self.pc += 2;
            }
            BDV => {
                self.b = self.a / (1 << self.combo(arg));
                self.pc += 2;
            }
            CDV => {
                self.c = self.a / (1 << self.combo(arg));
                self.pc += 2;
            }

            _ => panic!(),
        }
        true
    }
}

impl FromStr for Machine {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        let a = fixed_numbers::<u64, 1>(lines.next().ok_or("err")?)[0];
        let b = fixed_numbers::<u64, 1>(lines.next().ok_or("err")?)[0];
        let c = fixed_numbers::<u64, 1>(lines.next().ok_or("err")?)[0];
        lines.next().ok_or("err")?;
        let prog: Vec<u64> = numbers(lines.next().ok_or("err")?);
        Ok(Machine {
            a,
            b,
            c,
            pc: 0,
            prog,
            stdout: Default::default(),
        })
    }
}

fn main() {
    let machine: Machine = stdin_string().parse().unwrap();

    let mut p1 = machine.clone();
    while p1.step() {}
    println!(
        "{}",
        p1.stdout
            .iter()
            .map(|a| a.to_string())
            .collect::<Vec<_>>()
            .join(",")
    );

    // Special case for the example, we can just brute-force P2
    if machine.a == 2024 {
        for a in 0.. {
            let mut p2 = machine.clone();
            p2.a = a;

            while p2.step() {}
            if p2.stdout == machine.prog {
                println!("{a}");
                return;
            }
        }
    }

    // P2 solution is custom made for main input, don't try it on examples.
    if machine.a == 729 {
        return;
    }

    // Basic idea for P2: Register a is shifted right by an octal every round.
    // The last output octal only depends on the highest octal in a (only
    // thing left by then), but the previous ones are mangled with the higher
    // octals. Keep looking for good high octals that produce the correct
    // output suffix, and backtrack to increment them when it's not possible
    // to proceed with the current suffix.

    // Reversed octals for a.
    let mut octals = vec![0];

    loop {
        // We failed with current branch, backtrack.
        if octals[octals.len() - 1] == 8 {
            octals.pop();
            let n = octals.len() - 1;
            octals[n] += 1;
            continue;
        }

        let a = octals.iter().fold(0, |acc, n| (acc << 3) + n);
        let mut p2 = machine.clone();
        p2.a = a;
        while p2.step() {}

        let shared_suffix = p2
            .stdout
            .iter()
            .rev()
            .zip(machine.prog.iter().rev())
            .take_while(|(a, b)| a == b)
            .count();
        match shared_suffix {
            x if x == machine.prog.len() => {
                // Solution found.
                println!("{a}");
                break;
            }
            x if x >= octals.len() => {
                // Entire local suffix matches, go deeper.
                octals.push(0);
            }
            _ => {
                // Mismatch, increment top item.
                let n = octals.len() - 1;
                octals[n] += 1;
            }
        }
    }
}
