// NB. This solution involves a reverse-engineered optimized function from
// eyeballing the input assembly code. It might not generalize to other
// inputs.

use serde::Deserialize;

use aoc::{prelude::*, Operand};

#[derive(Copy, Clone, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
enum Opcode {
    Set(char, Operand),
    Sub(char, Operand),
    Mul(char, Operand),
    Jnz(Operand, Operand),
}

impl Opcode {
    /// Return relative PC jump after eval.
    pub fn eval(&self, regs: &mut [i64]) -> i32 {
        match self {
            Opcode::Set(r, a) => regs[reg(*r)] = a.val(regs),
            Opcode::Sub(r, a) => regs[reg(*r)] -= a.val(regs),
            Opcode::Mul(r, a) => regs[reg(*r)] *= a.val(regs),
            Opcode::Jnz(a, b) => {
                if a.val(regs) != 0 {
                    return b.val(regs) as i32;
                }
            }
        }
        1
    }
}

fn main() {
    let prog: Vec<Opcode> = idm::from_str(&stdin_string()).unwrap();

    // P1
    {
        let mut pc = 0;
        let mut regs = [0; 8];
        let mut muls = 0;
        while (0..prog.len() as i32).contains(&pc) {
            if matches!(prog[pc as usize], Opcode::Mul(_, _)) {
                muls += 1;
            }
            pc += prog[pc as usize].eval(&mut regs);
        }

        println!("{muls}");
    }

    // P2
    {
        // Run the start of the program to extract initial parameters.
        let mut pc = 0;
        let mut regs = [0; 8];
        regs[0] = 1;
        // Once it starts touching registers beyond b and c, stop.
        while regs[3] == 0 {
            pc += prog[pc as usize].eval(&mut regs);
        }
        let (b, c) = (regs[1], regs[2]);

        // At this point you stare at the assembly code until you figure out
        // it's running a very ineffective primality test. Then just write out
        // that:
        println!("{}", (b..=c).step_by(17).filter(|&a| !is_prime(a)).count());
    }
}
