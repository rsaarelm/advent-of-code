use aoc::prelude::*;

fn fix(seq: &[i32], rules: &[[i32; 2]]) -> Vec<i32> {
    let mut ret = seq.to_owned();
    let mut changes = 0;
    loop {
        for [a, b] in rules {
            if let (Some(a), Some(b)) = (idx_of(&ret, a), idx_of(&ret, b)) {
                if a > b {
                    ret.swap(a, b);
                    changes += 1;
                }
            }
        }

        if changes == 0 {
            return ret;
        } else {
            changes = 0;
        }
    }
}

fn main() {
    let mut iter = stdin_lines();

    let mut rules: Vec<[i32; 2]> = Vec::new();
    for line in iter.by_ref().take_while(|a| !a.is_empty()) {
        rules.push(fixed_numbers(line));
    }

    let mut updates: Vec<Vec<i32>> = Vec::new();
    for line in iter {
        updates.push(numbers(line));
    }

    let mut p1 = 0;
    let mut p2 = 0;

    for x in &updates {
        let update = fix(x, &rules);
        let mid_page = update[update.len() / 2];
        if &update == x {
            p1 += mid_page;
        } else {
            p2 += mid_page;
        }
    }

    println!("{p1}");
    println!("{p2}");
}
