use aoc::prelude::*;

fn deliver(target: usize, c: usize, elfstop: usize) -> usize {
    let mut bins = vec![0; target / 10]; // Guesstimated contracting.
    let n = bins.len();
    for i in 1..n {
        for j in (i..n).step_by(i).take(elfstop) {
            bins[j] += i * c;
            if bins[j] >= target {
                break;
            }
        }
    }
    bins.iter()
        .position(|&n| n >= target)
        .expect("Failed to reach target")
}

fn main() {
    let target = stdin_string().parse::<usize>().unwrap();
    println!("{}", deliver(target, 10, usize::MAX));
    println!("{}", deliver(target, 11, 50));
}
