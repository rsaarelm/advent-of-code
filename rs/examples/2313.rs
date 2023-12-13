use aoc::prelude::*;

fn x_reflect_error(bounds: &Rect<i32>, buf: &[char], x: i32) -> usize {
    let mut err = 0;
    for i in 0..x {
        let x1 = i;
        let x2 = x + (x - i - 1);
        if !bounds.contains([x2, 0]) {
            continue;
        }
        for y in 0..bounds.height() {
            if buf[bounds.idx([x1, y])] != buf[bounds.idx([x2, y])] {
                err += 1;
            }
        }
    }

    err
}

fn y_reflect_error(bounds: &Rect<i32>, buf: &[char], y: i32) -> usize {
    let mut err = 0;
    for i in 0..y {
        let y1 = i;
        let y2 = y + (y - i - 1);
        if !bounds.contains([0, y2]) {
            continue;
        }
        for x in 0..bounds.width() {
            if buf[bounds.idx([x, y1])] != buf[bounds.idx([x, y2])] {
                err += 1;
            }
        }
    }

    err
}

fn main() {
    let input: Vec<(Rect<i32>, Vec<char>)> =
        stdin_string().split("\n\n").map(grid).collect();

    for err in [0, 1] {
        let mut n = 0;
        'bounds: for (bounds, buf) in &input {
            for x in 1..bounds.width() {
                if x_reflect_error(bounds, buf, x) == err {
                    n += x;
                    continue 'bounds;
                }
            }

            for y in 1..bounds.height() {
                if y_reflect_error(bounds, buf, y) == err {
                    n += y * 100;
                    continue 'bounds;
                }
            }
        }

        println!("{n}");
    }
}
