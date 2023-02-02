use serde::Deserialize;
use serde_with::DeserializeFromStr;

use aoc::prelude::*;

#[derive(Copy, Clone, Debug, DeserializeFromStr)]
enum Operand {
    Int(i32),
    Reg(char),
}

use Operand::*;

impl Operand {
    fn val(&self, regs: &[i32; 4]) -> i32 {
        match self {
            Int(n) => *n,
            Reg(c) => regs[reg(*c)],
        }
    }
}

fn reg(c: char) -> usize {
    (c as u8 - 'a' as u8) as usize
}

impl FromStr for Operand {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<i32>() {
            Ok(n) => Ok(Int(n)),
            _ => Ok(Reg(s.chars().next().unwrap())),
        }
    }
}

#[derive(Copy, Clone, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
enum Opcode {
    Cpy(Operand, Operand),
    Jnz(Operand, Operand),
    Inc(char),
    Dec(char),
    Tgl(char),
    Out(Operand),
}

use Opcode::*;

impl Opcode {
    fn toggle(&mut self) {
        *self = match self {
            Cpy(a, b) => Jnz(*a, *b),
            Jnz(a, b) => Cpy(*a, *b),
            Inc(a) => Dec(*a),
            Dec(a) => Inc(*a),
            Tgl(a) => Inc(*a),
            Out(a) => Out(*a),
        }
    }
}

pub struct Cpu<'a> {
    pc: i32,
    regs: [i32; 4],
    prog: &'a mut Program,
}

impl<'a> Cpu<'a> {
    pub fn new(prog: &'a mut Program, regs: [i32; 4]) -> Self {
        Cpu { pc: 0, regs, prog }
    }

    pub fn run(&mut self) {
        for _ in self {}
    }
}

impl<'a> Iterator for Cpu<'a> {
    type Item = Option<i32>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pc < 0 || self.pc >= self.prog.0.len() as i32 {
            return None;
        }

        self.pc += 1;
        match self.prog.0[(self.pc - 1) as usize] {
            Cpy(_, Int(_)) => {}
            Cpy(a, Reg(b)) => self.regs[reg(b)] = a.val(&self.regs),
            Jnz(a, b) => {
                if a.val(&self.regs) != 0 {
                    self.pc += b.val(&self.regs) - 1;
                }
            }
            Inc(a) => self.regs[reg(a)] += 1,
            Dec(a) => self.regs[reg(a)] -= 1,
            Tgl(a) => {
                let i = self.pc + self.regs[reg(a)] - 1;
                if i >= 0 && i < self.prog.0.len() as i32 {
                    Opcode::toggle(&mut self.prog.0[i as usize]);
                }
            }
            Out(a) => {
                return Some(Some(a.val(&self.regs)));
            }
        }

        Some(None)
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct Program(Vec<Opcode>);

impl Program {
    pub fn run(&mut self, regs: &mut [i32; 4]) {
        let mut cpu = Cpu::new(self, *regs);
        cpu.run();
        *regs = cpu.regs;
    }
}
