use aoc::prelude::*;

fn main() {
    let input: Vec<i32> = stdin_lines_as().collect();

    for f in [|a| a + 1, |a| if a >= 3 { a - 1 } else { a + 1 }] {
        let mut ops = input.clone();
        let mut pc = 0;
        for i in 1.. {
            let n = ops[pc as usize];
            ops[pc as usize] = f(n);
            pc += n;
            if pc < 0 || pc >= ops.len() as i32 {
                println!("{i}");
                break;
            }
        }
    }
}
