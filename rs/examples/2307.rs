use aoc::prelude::*;

const ORDER_1: &str = "23456789TJQKA";
const ORDER_2: &str = "J23456789TQKA";

fn score(order: &str, s: &str) -> Vec<usize> {
    s.chars().map(|c| order.find(c).unwrap()).collect()
}

fn type_1(s: &str) -> usize {
    match &histogram(s.chars()).map(|(_, a)| a).collect::<Vec<_>>()[..] {
        [1, 1, 1, 1, 1] => 0,
        [2, 1, 1, 1] => 1,
        [2, 2, 1] => 2,
        [3, 1, 1] => 3,
        [3, 2] => 4,
        [4, 1] => 5,
        [5] => 6,
        _ => panic!(),
    }
}

fn jokers(s: &str) -> usize {
    s.chars().filter(|&c| c == 'J').count()
}

fn type_2(s: &str) -> usize {
    match (jokers(s), type_1(s)) {
        (1, 0) => 1,
        (1 | 2, 1) => 3,
        (1, 2) => 4,
        (2, 2) => 5,
        (1 | 3, 3) => 5,
        (3 | 2, 4) => 6,
        (1 | 4, 5) => 6,
        (5, 6) => 6,
        (0, a) => a,
        _ => panic!(),
    }
}

fn main() {
    let input: Vec<(String, usize)> =
        parsed_stdin_lines(r"(.*) (.*)").collect();

    let mut p1 = input.clone();
    p1.sort_by_key(|(h, _)| (type_1(h), score(ORDER_1, h)));

    let mut p2 = input.clone();
    p2.sort_by_key(|(h, _)| (type_2(h), score(ORDER_2, h)));

    for input in [p1, p2] {
        println!(
            "{}",
            input
                .iter()
                .enumerate()
                .map(|(i, (_, n))| (i + 1) * n)
                .sum::<usize>()
        );
    }
}
