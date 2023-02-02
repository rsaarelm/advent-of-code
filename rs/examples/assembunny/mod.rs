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
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct Program(Vec<Opcode>);

impl Program {
    pub fn run(&mut self, regs: &mut [i32; 4]) {
        let mut pc = 0;
        let prog = &mut self.0;

        while pc >= 0 && pc < prog.len() as i32 {
            match prog[pc as usize] {
                Cpy(_, Int(_)) => {}
                Cpy(a, Reg(b)) => regs[reg(b)] = a.val(&regs),
                Jnz(a, b) => {
                    if a.val(&regs) != 0 {
                        pc += b.val(&regs);
                        continue;
                    }
                }
                Inc(a) => regs[reg(a)] += 1,
                Dec(a) => regs[reg(a)] -= 1,
                Tgl(a) => {
                    let i = pc + regs[reg(a)];
                    if i >= 0 && i < prog.len() as i32 {
                        Opcode::toggle(&mut prog[i as usize]);
                    }
                }
            }
            pc += 1;
        }
    }
}
