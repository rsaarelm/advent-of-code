use std::collections::BTreeMap;

use aoc::prelude::*;

fn quints(input: &str) -> impl Iterator<Item = Vec<u8>> + '_ {
    let mut i = 0;
    std::iter::from_fn(move || loop {
        let hash = bytes_to_hex(&md5sum(format!("{input}{i}").as_bytes()));
        i += 1;
        if i % 8192 == 0 {
            eprint!("\x1b[1;31m{hash}\x1b[0m    \r");
        }
        if hash.starts_with("00000") {
            return Some(hash.into_bytes());
        }
    })
}

fn crack(input: &str) -> String {
    let mut password = String::new();
    for hash in quints(input).take(8) {
        eprintln!(
            "{}\x1b[1;30m{}\x1b[0m\x1b[1;32m{}\x1b[0m{}",
            String::from_utf8(vec![b' '; password.len()]).unwrap(),
            std::str::from_utf8(&hash[..5]).unwrap(),
            hash[5] as char,
            std::str::from_utf8(&hash[6..]).unwrap()
        );
        password.push((hash[5] as char).to_ascii_lowercase());
    }
    password
}

fn advanced_crack(input: &str) -> String {
    let mut password = BTreeMap::default();
    for hash in quints(input) {
        let index = (hash[5] as char).to_digit(16).unwrap();
        if index >= 8 || password.contains_key(&index) {
            continue;
        }
        eprintln!(
            "{}\x1b[1;30m{}\x1b[0m\x1b[1;32m{}\x1b[0m{}",
            String::from_utf8(vec![b' '; index as usize]).unwrap(),
            std::str::from_utf8(&hash[..6]).unwrap(),
            hash[6] as char,
            std::str::from_utf8(&hash[7..]).unwrap()
        );
        password.insert(index, (hash[6] as char).to_ascii_lowercase());

        if password.len() >= 8 {
            return password.values().collect();
        }
    }
    unreachable!()
}

fn main() {
    let input = stdin_string();
    eprintln!("Executing hack...");
    println!("{}", crack(&input));
    eprintln!("Executing advanced hack...");
    println!("{}", advanced_crack(&input));
}
