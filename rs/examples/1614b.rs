use itertools::Itertools;
use memoize::memoize;

use aoc::prelude::*;

#[memoize]
fn hash(salt: String, stretch: usize, i: usize) -> String {
    let mut s = bytes_to_hex(&md5sum(format!("{salt}{i}").as_bytes()));
    for _ in 0..stretch {
        s = bytes_to_hex(&md5sum(s.as_bytes()));
    }
    s
}

#[memoize]
fn quints(salt: String, stretch: usize, i: usize) -> String {
    let mut ret = String::new();
    let key = hash(salt, stretch, i);

    for (n, c) in key.chars().dedup_with_count() {
        if n >= 5 {
            ret.push(c);
        }
    }

    ret
}

fn trips(s: &str) -> Option<char> {
    for (n, c) in s.chars().dedup_with_count() {
        if n >= 3 {
            return Some(c);
        }
    }
    None
}

fn main() {
    let input = stdin_string();

    for stretch in [0, 2016] {
        let mut keys_found = 0;

        for i in 0.. {
            let key = hash(input.to_owned(), stretch, i);
            if let Some(c) = trips(&key) {
                for j in (i + 1)..(i + 1001) {
                    if quints(input.to_owned(), stretch, j).contains(c) {
                        keys_found += 1;
                        break;
                    }
                }
            }
            if keys_found == 64 {
                println!("{i}");
                break;
            }
        }
    }
}
