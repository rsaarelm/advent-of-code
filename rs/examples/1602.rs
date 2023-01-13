use aoc::prelude::*;

fn main() {
    let input: Vec<Vec<IVec2>> = stdin_lines()
        .map(|line| {
            line.chars()
                .map(|c| DIR_4["RDLU".find(c).unwrap()])
                .collect()
        })
        .collect();

    // Part 1
    let bounds = area(3, 3);
    let mut pos = ivec2(1, 1);

    for seq in &input {
        for &step in seq {
            pos = bounds.clamp(pos + step);
        }
        print!("{}", bounds.index_of(pos) + 1);
    }
    println!();

    // Part 2
    let mut pos = ivec2(-2, 0);
    let bounds = area(5, 5) - ivec2(2, 2);
    let pad = "  1   234 56789 ABC   D  ".as_bytes();

    for seq in &input {
        for &step in seq {
            if (pos + step).taxi_len() <= 2 {
                pos += step;
            }
        }
        print!("{}", pad[bounds.index_of(pos)] as char);
    }
    println!();
}
