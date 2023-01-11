use aoc::prelude::*;
use itertools::Itertools;

fn main() {
    let mut input = stdin_string().into_bytes();
    for i in 1..51 {
        input = input
            .into_iter()
            .dedup_with_count()
            .flat_map(|(n, c)| format!("{n}{}", c as char).into_bytes())
            .collect();
        if i == 40 {
            println!("{}", input.len());
        }
        if i == 50 {
            println!("{}", input.len());
        }
    }
}
