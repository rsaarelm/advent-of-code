use aoc::prelude::*;

fn main() {
    let input: Vec<i32> = stdin_string()
        .chars()
        .map(|c| ") (".find(c).unwrap() as i32 - 1)
        .collect();

    println!("{}", input.iter().sum::<i32>());

    let mut n = 0;
    for (i, c) in input.iter().enumerate() {
        n += c;
        if n < 0 {
            println!("{}", i + 1);
            break;
        }
    }
}
