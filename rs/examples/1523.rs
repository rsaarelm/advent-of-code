use aoc::prelude::*;

#[derive(Debug)]
enum Op {
    Hlf(usize),
    Tpl(usize),
    Inc(usize),
    Jmp(i32),
    Jie(usize, i32),
    Jio(usize, i32),
}

use Op::*;

impl FromStr for Op {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = (&s[..4], &s[4..]);
        fn r(s: &str) -> usize {
            s.chars().next().unwrap() as usize - 'a' as usize
        }
        match a {
            "hlf " => Ok(Hlf(r(b))),
            "tpl " => Ok(Tpl(r(b))),
            "inc " => Ok(Inc(r(b))),
            "jmp " => Ok(Jmp(fixed_numbers::<i32, 1>(b)[0])),
            "jie " => Ok(Jie(r(b), fixed_numbers::<i32, 1>(b)[0])),
            "jio " => Ok(Jio(r(b), fixed_numbers::<i32, 1>(b)[0])),
            _ => Err(()),
        }
    }
}

impl Op {
    fn exec(&self, regs: &mut [u32]) -> i32 {
        match *self {
            Hlf(i) => regs[i] /= 2,
            Tpl(i) => regs[i] *= 3,
            Inc(i) => regs[i] += 1,
            Jmp(d) => return d,
            Jie(i, d) => {
                if regs[i] % 2 == 0 {
                    return d;
                }
            }
            Jio(i, d) => {
                if regs[i] == 1 {
                    return d;
                }
            }
        }

        1
    }
}

fn run(mem: &[Op], init: &[u32]) -> u32 {
    let mut regs = init.to_vec();
    let mut pc = 0;
    let range = 0..(mem.len() as i32);
    while range.contains(&pc) {
        pc += mem[pc as usize].exec(&mut regs);
    }
    regs[1]
}

fn main() {
    let mem: Vec<Op> = stdin_lines_as().collect();

    println!("{}", run(&mem, &[0, 0]));
    println!("{}", run(&mem, &[1, 0]));
}
