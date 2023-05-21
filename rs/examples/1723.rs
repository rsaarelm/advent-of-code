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
        // Extract parameters.
        let mut pc = 0;
        let mut regs = [0; 8];
        regs[0] = 1;
        while regs[3] == 0 {
            pc += prog[pc as usize].eval(&mut regs);
        }
        let (mut b, c) = (regs[1], regs[2]);

        let mut d;
        let mut e;
        let mut f;
        let mut g;
        let mut h = 0;
        // Write out the rest of the code.
        loop {
            f = 1;
            d = 2;                  // d Starts as 2
            loop {
                e = 2;
                loop {              // Loop increments e
                    g = d;          // g initialized with d
                    g *= e;
                    g -= b;         // d * e == b
                    if g == 0 {     // f control's h's increment
                        f = 0;
                    }
                    e += 1;
                    g = e;          // e needs to count up to b for break
                    g -= b;
                    if g == 0 {
                        break;
                    }
                }
                d += 1;             // d incremented
                g = d;
                g -= b;
                if g == 0 {
                    break;
                }
            }
            if f == 0 {
                h += 1;
            }
            g = b;
            g -= c;
            if g == 0 {
                break;
            }

            b += 17;                // b steps 1000 times to reach c
        }

        println!("{h}");
    }

    // P2 (naive version, will not finish)
    /*
    {
        let mut pc = 0;
        let mut regs = [0; 8];
        regs[0] = 1;
        while (0..prog.len() as i32).contains(&pc) {
            pc += prog[pc as usize].eval(&mut regs);
        }

        println!("{}", regs[(b'h' - b'a') as usize]);
    }
    */
}
