use aoc::prelude::*;

mod assembunny;

fn main() {
    let prog: assembunny::Program = idm::from_str(&stdin_string()).unwrap();

    'calibration: for i in 0.. {
        let mut p = prog.clone();
        let cpu = assembunny::Cpu::new(&mut p, [i, 0, 0, 0]);
        for (x, y) in cpu.flatten().enumerate() {
            if y != (x as i64) % 2 {
                break;
            }
            if x > 4096 {
                println!("{i}");
                break 'calibration;
            }
        }
    }
}
