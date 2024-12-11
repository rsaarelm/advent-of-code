use aoc::prelude::*;

fn blink(stones: &HashMap<u64, usize>) -> HashMap<u64, usize> {
    let mut new_stones = HashMap::default();
    for (i, n) in stones.iter().map(|(&a, &b)| (a, b)).collect::<Vec<_>>() {
        let digits = format!("{}", i);
        if i == 0 {
            *new_stones.entry(1).or_default() += n;
        } else if digits.len() % 2 == 0 {
            let (a, b) = (
                digits[0..digits.len() / 2].parse::<u64>().unwrap(),
                digits[digits.len() / 2..].parse::<u64>().unwrap(),
            );
            *new_stones.entry(a).or_default() += n;
            *new_stones.entry(b).or_default() += n;
        } else {
            *new_stones.entry(i * 2024).or_default() += n;
        }
    }
    new_stones
}

fn main() {
    let mut stones: HashMap<u64, usize> =
        histogram(numbers(stdin_string())).collect();

    (0..25).for_each(|_| stones = blink(&stones));
    println!("{}", stones.values().sum::<usize>());

    (25..75).for_each(|_| stones = blink(&stones));
    println!("{}", stones.values().sum::<usize>());
}
