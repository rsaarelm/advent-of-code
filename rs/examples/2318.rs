use aoc::prelude::*;
use glam::{i64vec2, I64Vec2};

fn survey(ops: &[(i64, I64Vec2)]) -> i64 {
    let mut left = Vec::new();
    let mut right = Vec::new();

    let mut left_pos = i64vec2(0, 0);
    let mut right_pos = i64vec2(0, 0);
    // Trace the polygons for both sides of the thick line being drawn.
    for (i, &(n, dir)) in ops.iter().enumerate() {
        let next_dir = ops[(i + 1) % ops.len()].1;
        if next_dir == dir {
            // Straight.
            left_pos += n * dir;
            left.push(left_pos);

            right_pos += n * dir;
            right.push(right_pos);
        } else if next_dir == dir.ccw() {
            // Left turn
            left_pos += (n - 1) * dir;
            left.push(left_pos);

            right_pos += n * dir;
            right.push(right_pos);
            right_pos += next_dir;
        } else if next_dir == dir.cw() {
            // Right turn
            left_pos += n * dir;
            left.push(left_pos);
            left_pos += next_dir;

            right_pos += (n - 1) * dir;
            right.push(right_pos);
        } else {
            panic!()
        }
    }

    // The bigger one is the outer rim.
    polygon_area(&left).max(polygon_area(&right))
}

fn main() {
    let mut p1 = Vec::new();
    let mut p2 = Vec::new();

    for (dir1, n1, n2, dir2) in parsed_stdin_lines::<(char, i64, String, String)>(
        r"(.*) (.*) \(#(.....)(.)\)",
    ) {
        p1.push((
            n1,
            match dir1 {
                'U' => DIR_4[UP],
                'L' => DIR_4[LEFT],
                'R' => DIR_4[RIGHT],
                'D' => DIR_4[DOWN],
                _ => panic!(),
            }
            .as_i64vec2(),
        ));

        p2.push((
            i64::from_str_radix(&n2, 16).unwrap(),
            match u8::from_str_radix(&dir2, 16).unwrap() {
                0 => DIR_4[RIGHT],
                1 => DIR_4[DOWN],
                2 => DIR_4[LEFT],
                3 => DIR_4[UP],
                _ => panic!(),
            }
            .as_i64vec2(),
        ));
    }

    println!("{}", survey(&p1));
    println!("{}", survey(&p2));
}
