use aoc::prelude::*;

#[memoize]
fn valid_fills(s: String, slots: Vec<usize>) -> usize {
    if slots.is_empty() {
        if s.contains('#') {
            return 0;
        } else {
            return 1;
        }
    }

    // Remaining slots can't fit in string.
    if s.len() < slots.iter().sum::<usize>() + slots.len() - 1 {
        return 0;
    }

    match s.chars().next() {
        // Jump over points.
        Some('.') => valid_fills(s[1..].to_owned(), slots),
        // Next slot must fit here.
        Some('#') => {
            if s.len() >= slots[0]
                && s.chars().take(slots[0]).all(|c| c != '.')
                && s.chars().nth(slots[0]).map_or(true, |c| c != '#')
            {
                valid_fills(
                    s[(slots[0] + 1).min(s.len())..].to_owned(),
                    slots[1..].to_owned(),
                )
            } else {
                // Does not fit, fail here.
                0
            }
        }
        Some('?') => {
            let mut n = 0;
            if s.len() >= slots[0]
                && s.chars().take(slots[0]).all(|c| c != '.')
                && s.chars().nth(slots[0]).map_or(true, |c| c != '#')
            {
                n += valid_fills(
                    s[(slots[0] + 1).min(s.len())..].to_owned(),
                    slots[1..].to_owned(),
                );
            }

            n += valid_fills(s[1..].to_owned(), slots);

            n
        }
        _ => panic!(),
    }
}

fn main() {
    let input: Vec<(String, Vec<usize>)> = stdin_lines()
        .map(|line| {
            let (a, b) = line.split_once(' ').unwrap();
            (a.to_owned(), numbers(b))
        })
        .collect();

    let input_2: Vec<(String, Vec<usize>)> = input
        .iter()
        .map(|(s, v)| {
            let len = v.len() * 5;
            (
                format!("{s}?{s}?{s}?{s}?{s}"),
                v.iter().cycle().take(len).copied().collect(),
            )
        })
        .collect();

    for i in [input, input_2] {
        let mut n = 0;
        for (a, set) in i {
            n += valid_fills(a.clone(), set.clone());
        }
        println!("{n}");
    }
}
