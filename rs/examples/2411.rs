use aoc::prelude::*;

fn blink(stones: &mut HashMap<u64, usize>) {
    for (i, n) in stones.iter().map(|(&a, &b)| (a, b)).collect::<Vec<_>>() {
        if stones[&i] == n {
            stones.remove(&i);
        } else {
            stones.insert(i, stones[&i] - n);
        }

        let digits = format!("{}", i);
        if i == 0 {
            *stones.entry(1).or_default() += n;
        } else if digits.len() % 2 == 0 {
            let (a, b) = (
                digits[0..digits.len() / 2].parse::<u64>().unwrap(),
                digits[digits.len() / 2..].parse::<u64>().unwrap(),
            );
            *stones.entry(a).or_default() += n;
            *stones.entry(b).or_default() += n;
        } else {
            *stones.entry(i * 2024).or_default() += n;
        }
    }
}

fn main() {
    let mut stones: HashMap<u64, usize> =
        histogram(numbers(stdin_string())).collect();

    (0..25).for_each(|_| blink(&mut stones));
    println!("{}", stones.values().sum::<usize>());

    (25..75).for_each(|_| blink(&mut stones));
    println!("{}", stones.values().sum::<usize>());
}
