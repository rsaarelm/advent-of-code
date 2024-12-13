use aoc::prelude::*;

fn main() {
    let mut n = 0;
    let mut machines = Vec::new();
    for line in stdin_lines() {
        match &numbers::<i64>(line)[..] {
            &[a, b] if n == 0 => {
                machines.push(([a, b], [0, 0], [0, 0]));
                n += 1;
            }
            &[a, b] if n == 1 => {
                let i = machines.len() - 1;
                machines[i].1 = [a, b];
                n += 1;
            }
            &[a, b] if n == 2 => {
                let i = machines.len() - 1;
                machines[i].2 = [a, b];
                n += 1;
            }
            [] => {
                n = 0;
            }
            _ => panic!(),
        }
    }

    let mut p1 = 0;
    let mut p2 = 0;

    for &([ax, ay], [bx, by], [tx, ty]) in &machines {
        if let Some(sln) = solve_integer_linear_system(&[ax, bx, ay, by], &[tx, ty]) {
             p1 += 3 * sln[0] + sln[1];
        }

        const P2_D: i64 = 10000000000000;
        if let Some(sln) = solve_integer_linear_system(&[ax, bx, ay, by], &[tx + P2_D, ty + P2_D]) {
             p2 += 3 * sln[0] + sln[1];
        }
    }

    println!("{p1}");
    println!("{p2}");
}
