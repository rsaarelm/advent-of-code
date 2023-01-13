use aoc::prelude::*;

fn press(
    (bounds, grid): &(NRange<i32, 2>, Vec<char>),
    prev: char,
    code: &str,
) -> char {
    let prev_pos = grid.iter().position(|&p| p == prev).unwrap();
    let mut pos: IVec2 = bounds.get(prev_pos).into();

    for d in code.chars().map(|c| DIR_4["RDLU".find(c).unwrap()]) {
        let p = bounds.clamp(pos + d);
        if !grid[bounds.idx(p)].is_whitespace() {
            pos = p;
        }
    }

    grid[bounds.idx(pos)]
}

fn main() {
    let input: Vec<String> = stdin_lines().collect();

    let p1 = flatgrid(
        "
123
456
789",
    );

    let p2 = flatgrid(
        "
  1
 234
56789
 ABC
  D",
    );

    for pad in [p1, p2] {
        let mut k = '5';
        for line in &input {
            k = press(&pad, k, line);
            print!("{k}");
        }
        println!();
    }
}
