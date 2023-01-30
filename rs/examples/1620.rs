use aoc::prelude::*;

fn main() {
    let mut input: Vec<_> =
        parsed_stdin_lines::<(u32, u32)>(r"(\d+)-(\d+)").collect();
    input.sort();

    let mut i = 1;
    while i < input.len() {
        if input[i - 1].1 >= input[i].0 - 1 {
            input[i - 1].1 = input[i - 1].1.max(input[i].1);
            input.remove(i);
        } else {
            i += 1;
        }
    }

    println!("{}", input[0].1 + 1);

    // XXX: Assumes 0 and 0xffffffff are covered by intervals.
    println!(
        "{}",
        input
            .iter()
            .zip(input.iter().skip(1))
            .map(|((_, a), (b, _))| b - a - 1)
            .sum::<u32>()
    );
}
