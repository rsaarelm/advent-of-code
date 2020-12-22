use std::collections::{BTreeSet, HashMap};

fn part_1(seq: &BTreeSet<i32>) -> u32 {
    let mut ones = 0;
    let mut threes = 0;
    for (a, b) in seq.iter().zip(seq.iter().skip(1)) {
        if b - a == 1 {
            ones += 1;
        } else {
            threes += 1;
        }
    }

    ones * threes
}

fn part_2(seq: &BTreeSet<i32>) -> u64 {
    fn paths_at(cache: &mut HashMap<i32, u64>, seq: &BTreeSet<i32>, n: i32) -> u64 {
        if let Some(&c) = cache.get(&n) {
            // Memoizing cache
            return c;
        }

        let ret = &[1, 2, 3] // Can take max 3 steps in graph
            .iter()
            .map(|c| n + c)
            .filter(|n| seq.contains(n)) // Can only step on existing numbers
            .map(|n| paths_at(cache, seq, n))
            .sum(); // Add up variant paths up ahead
        let ret = if *ret == 0 { 1 } else { *ret }; // Unit path on reaching end
        cache.insert(n, ret);
        ret
    }

    // Return count of different paths over the adapter DAG graph.
    paths_at(&mut HashMap::new(), seq, 0)
}

fn main() {
    let input: BTreeSet<i32> = {
        use std::{io, io::prelude::*};
        let mut ret: BTreeSet<i32> = io::stdin()
            .lock()
            .lines()
            .map(|line| line.unwrap().parse::<i32>().unwrap())
            .collect();
        ret.insert(0);
        ret.insert(ret.iter().max().unwrap() + 3);
        ret
    };

    println!("{:?}", part_1(&input));
    println!("{:?}", part_2(&input));
}
