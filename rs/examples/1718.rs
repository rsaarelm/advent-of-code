use std::collections::VecDeque;

use serde::Deserialize;

use aoc::{prelude::*, Operand};

#[derive(Copy, Clone, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
enum Opcode {
    Snd(char),
    Set(char, Operand),
    Add(char, Operand),
    Mul(char, Operand),
    Mod(char, Operand),
    Rcv(char),
    Jgz(Operand, Operand),
}

struct Cpu<'a> {
    pc: i64,
    regs: [i64; 26],
    input: VecDeque<i64>,
    prog: &'a [Opcode],
}

impl<'a> Cpu<'a> {
    pub fn new(id: i64, prog: &'a [Opcode]) -> Cpu<'a> {
        let mut ret = Cpu {
            pc: 0,
            regs: [0; 26],
            input: Default::default(),
            prog,
        };
        ret.regs[(b'p' - b'a') as usize] = id;
        ret
    }

    pub fn can_run(&self) -> bool {
        // CPU can't run if either it's program has terminated or it's
        // awaiting input. (We don't care which, terminating programs can just
        // be treated as a deadlock.)
        self.pc >= 0
            && self.pc < self.prog.len() as i64
            && !(matches!(self.prog[self.pc as usize], Opcode::Rcv(_))
                && self.input.is_empty())
    }

    /// Return number of sends done during run.
    pub fn run(&mut self, output: &mut VecDeque<i64>) -> usize {
        let n = output.len();
        while self.step(output) {}
        output.len() - n
    }

    // Correct P2 instructions.
    /// Return whether run can continue.
    fn step(&mut self, output: &mut VecDeque<i64>) -> bool {
        if self.pc < 0 || self.pc > self.prog.len() as i64 {
            return false;
        }
        match self.prog[self.pc as usize] {
            Opcode::Snd(a) => output.push_back(self.regs[reg(a)]),
            Opcode::Set(r, a) => self.regs[reg(r)] = a.val(&self.regs),
            Opcode::Add(r, a) => self.regs[reg(r)] += a.val(&self.regs),
            Opcode::Mul(r, a) => self.regs[reg(r)] *= a.val(&self.regs),
            Opcode::Mod(r, a) => self.regs[reg(r)] %= a.val(&self.regs),
            Opcode::Rcv(r) => {
                if let Some(a) = self.input.pop_front() {
                    self.regs[reg(r)] = a
                } else {
                    return false;
                }
            }
            Opcode::Jgz(a, b) => {
                if a.val(&self.regs) > 0 {
                    self.pc += b.val(&self.regs);
                    return true;
                }
            }
        }
        self.pc += 1;

        true
    }
}

fn main() {
    let prog: Vec<Opcode> = idm::from_str(&stdin_string()).unwrap();

    let mut a = Cpu::new(0, &prog);
    // Emulate the bad instruction set in P1.
    let mut sent = VecDeque::new();
    while a.can_run() {
        a.step(&mut sent);
        if let Opcode::Rcv(r) = a.prog[a.pc as usize] {
            if a.regs[reg(r)] != 0 {
                println!("{}", sent.pop_back().unwrap());
                break;
            } else {
                a.pc += 1;
            }
        }
    }

    // P2

    let mut a = Cpu::new(0, &prog);
    let mut b = Cpu::new(1, &prog);
    let mut b_sends = 0;
    while a.can_run() || b.can_run() {
        if a.can_run() {
            a.run(&mut b.input);
        } else {
            b_sends += b.run(&mut a.input);
        }
    }
    println!("{b_sends}");
}
