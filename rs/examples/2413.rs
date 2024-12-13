use aoc::prelude::*;

fn main() {
    let mut machines: Vec<([i64; 2], [i64; 2], [i64; 2])> = Vec::new();

    || -> Option<()> {
        let mut lines = stdin_lines();
        loop {
            machines.push((
                fixed_numbers(lines.next()?),
                fixed_numbers(lines.next()?),
                fixed_numbers(lines.next()?),
            ));
            lines.next()?;
        }
    }();

    let mut p1 = 0;
    let mut p2 = 0;

    for &([ax, ay], [bx, by], [tx, ty]) in &machines {
        if let Some(sln) = solve_linear_system(&[ax, bx, ay, by], &[tx, ty]) {
            p1 += 3 * sln[0] + sln[1];
        }

        const P2_D: i64 = 10000000000000;
        if let Some(sln) =
            solve_linear_system(&[ax, bx, ay, by], &[tx + P2_D, ty + P2_D])
        {
            p2 += 3 * sln[0] + sln[1];
        }
    }

    println!("{p1}");
    println!("{p2}");
}
