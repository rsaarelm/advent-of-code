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
    Cpy(Operand, char),
    Jnz(Operand, i32),
    Inc(char),
    Dec(char),
}

use Opcode::*;

fn run(prog: &[Opcode], regs: &mut [i32; 4]) {
    let mut pc = 0;

    while pc >= 0 && pc < prog.len() as i32 {
        match prog[pc as usize] {
            Cpy(a, b) => regs[reg(b)] = a.val(&regs),
            Jnz(a, n) => {
                if a.val(&regs) != 0 {
                    pc += n;
                    continue;
                }
            }
            Inc(a) => regs[reg(a)] += 1,
            Dec(a) => regs[reg(a)] -= 1,
        }
        pc += 1;
    }
}

fn main() {
    let prog: Vec<Opcode> = idm::from_str(&stdin_string()).unwrap();

    let mut regs = [0, 0, 0, 0];
    run(&prog, &mut regs);
    println!("{}", regs[0]);

    let mut regs = [0, 0, 1, 0];
    run(&prog, &mut regs);
    println!("{}", regs[0]);
}
