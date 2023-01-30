use aoc::prelude::*;

fn main() {
    let (bounds, buf) = stdin_grid();

    for x in 0..bounds.width() {
        print!(
            "{}",
            histogram((0..bounds.height()).map(|y| buf[bounds.idx([x, y])]))
                .next()
                .unwrap()
                .0
        );
    }
    println!();

    for x in 0..bounds.width() {
        print!(
            "{}",
            histogram((0..bounds.height()).map(|y| buf[bounds.idx([x, y])]))
                .last()
                .unwrap()
                .0
        );
    }
    println!();
}
