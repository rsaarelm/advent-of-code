use aoc::prelude::*;

fn main() {
    let input: Vec<Vec<u32>> = stdin_lines().map(numbers).collect();

    let mut p1 = 0;
    for tri in &input {
        let mut tri = tri.clone();
        tri.sort();
        if tri[0] + tri[1] > tri[2] {
            p1 += 1;
        }
    }
    println!("{p1}");

    let mut p2 = 0;
    for row in (0..input.len()).step_by(3) {
        for col in 0..3 {
            let mut tri =
                vec![input[row][col], input[row + 1][col], input[row + 2][col]];
            tri.sort();
            if tri[0] + tri[1] > tri[2] {
                p2 += 1;
            }
        }
    }
    println!("{p2}");
}
