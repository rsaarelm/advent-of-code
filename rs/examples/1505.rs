use aoc::prelude::*;

fn main() {
    let input = stdin_string();

    // Part 1
    let mut nice_1 = 0;

    'lines: for line in input.lines() {
        if line.chars().filter(|&c| "aeiou".contains(c)).count() < 3 {
            continue;
        }

        if !line.chars().zip(line.chars().skip(1)).any(|(a, b)| a == b) {
            continue;
        }

        for bad in ["ab", "cd", "pq", "xy"] {
            if line.contains(bad) {
                continue 'lines;
            }
        }

        nice_1 += 1;
    }

    println!("{nice_1}");

    // Part 2
    let mut nice_2 = 0;
    for line in input.lines() {
        let mut has_pair = false;
        let mut has_twin = false;

        for i in 0..(line.len() - 2) {
            if line[(i + 2)..].contains(&line[i..(i + 2)]) {
                has_pair = true;
            }
            let line = line.as_bytes();
            if line[i] == line[i + 2] {
                has_twin = true;
            }
        }
        if !has_pair || !has_twin {
            continue;
        }

        nice_2 += 1;
    }

    println!("{nice_2}");
}
