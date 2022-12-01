use aoc::prelude::*;

fn increase_count(data: &[u32]) -> usize {
    data.iter()
        .zip(data.iter().skip(1))
        .filter(|(a, b)| b > a)
        .count()
}

fn main() {
    let data: Vec<u32> = stdin_lines_as::<u32>().collect();

    println!("{}", increase_count(&data));

    let data3: Vec<u32> = data
        .iter()
        .zip(data.iter().skip(1).zip(data.iter().skip(2)))
        .map(|(a, (b, c))| a + b + c)
        .collect();

    println!("{}", increase_count(&data3));
}
