use aoc::prelude::*;

fn main() {
    let mut input = Vec::new();

    for line in stdin_lines() {
        let [_, n, _, x]: [usize; 4] = fixed_numbers(line);
        input.push((n, x));
    }

    for _p in [1, 2] {
        'search: for i in 0.. {
            for (j, (n, x)) in input.iter().enumerate() {
                if (x + i + j + 1) % n != 0 {
                    continue 'search;
                }
            }
            println!("{i}");
            break;
        }

        input.push((11, 0));
    }
}
