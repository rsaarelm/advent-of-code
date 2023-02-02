use aoc::prelude::*;

mod assembunny;

fn main() {
    let prog: assembunny::Program = idm::from_str(&stdin_string()).unwrap();

    let mut regs = [0, 0, 0, 0];
    prog.clone().run(&mut regs);
    println!("{}", regs[0]);

    let mut regs = [0, 0, 1, 0];
    prog.clone().run(&mut regs);
    println!("{}", regs[0]);
}
