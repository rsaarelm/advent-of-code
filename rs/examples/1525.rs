use aoc::prelude::*;

fn undiagonalize(x: usize, y: usize) -> usize {
    // Σi of diagonals up to (x+y-1),
    // then the span in current diagonal given by x.
    (x + y) * (x + y + 1) / 2 + x
}

fn main() {
    let [y, x] = fixed_numbers::<usize, 2>(stdin_string());
    println!(
        "{}",
        (0..undiagonalize(x - 1, y - 1))
            .fold(20151125u64, |acc, _| (acc * 252533) % 33554393)
    );
}
