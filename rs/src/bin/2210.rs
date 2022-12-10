use aoc::prelude::*;
use std::collections::HashMap;

struct VM {
    cycle: usize,
    delay: usize,
    register: i32,
    pending_ops: HashMap<usize, i32>,
}

impl Default for VM {
    fn default() -> Self {
        VM {
            cycle: 1,
            delay: 0,
            register: 1,
            pending_ops: Default::default(),
        }
    }
}

impl VM {
    pub fn tick(&mut self, op: Option<i32>) {
        if let Some(n) = op {
            self.delay += 1;
            self.pending_ops.insert(self.cycle + self.delay, n);
        }
        if let Some(n) = self.pending_ops.get(&self.cycle) {
            self.register += n;
        }
        self.cycle += 1;
    }

    pub fn signal_strength(&self) -> i32 {
        if self.cycle >= 20 && (self.cycle - 20) % 40 == 0 {
            self.cycle as i32 * self.register
        } else {
            0
        }
    }

    pub fn x(&self) -> i32 {
        (self.cycle % 40) as i32
    }
}

fn main() {
    let parser = re_parser::<i32>(r"^addx (.+)$");
    let input: Vec<Option<i32>> = stdin_lines().map(|line| parser(&line).ok()).collect();

    let mut vm = VM::default();
    let mut signals = 0;
    for &op in input.iter().cycle().take(220) {
        vm.tick(op);
        signals += vm.signal_strength();
    }
    println!("{}", signals);

    let mut vm = VM::default();
    let mut ops = input.iter().cycle();
    for y in 0..6 {
        for x in 0..40 {
            if (vm.x() - vm.register - 1).abs() <= 1 {
                eprint!("#");
            } else {
                eprint!(".");
            }
            vm.tick(*ops.next().unwrap());
        }
        eprintln!();
    }
}
