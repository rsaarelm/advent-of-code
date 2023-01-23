use aoc::prelude::*;

#[derive(Copy, Clone, Debug)]
enum Op {
    Rect(i32, i32),
    RotateRow(i32, i32),
    RotateCol(i32, i32),
}

use Op::*;

impl FromStr for Op {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let [a, b] = fixed_numbers(s);
        if s.starts_with("rect ") {
            Ok(Rect(a, b))
        } else if s.starts_with("rotate row ") {
            Ok(RotateRow(a, b))
        } else if s.starts_with("rotate column ") {
            Ok(RotateCol(a, b))
        } else {
            Err(())
        }
    }
}

fn main() {
    let bounds = area(50, 6);

    let input: Vec<Op> = stdin_lines_as().collect();
    let mut buf = vec![0; bounds.volume() as usize];

    for &op in &input {
        match op {
            Rect(x, y) => {
                for p in area(x, y) {
                    buf[bounds.idx(p)] = 1;
                }
            }
            RotateRow(y, n) => {
                let row: Vec<u8> = (0..bounds.width())
                    .map(|x| buf[bounds.idx([x - n, y])])
                    .collect();
                for x in 0..bounds.width() {
                    buf[bounds.idx([x, y])] = row[x as usize];
                }
            }
            RotateCol(x, n) => {
                let col: Vec<u8> = (0..bounds.height())
                    .map(|y| buf[bounds.idx([x, y - n])])
                    .collect();
                for y in 0..bounds.height() {
                    buf[bounds.idx([x, y])] = col[y as usize];
                }
            }
        }
    }

    let pixels: Vec<[i32; 2]> = bounds
        .into_iter()
        .filter(|&p| buf[bounds.idx(p)] != 0)
        .collect();

    println!("{}", pixels.len());
    println!("{}", ocr(&pixels).unwrap());
}
