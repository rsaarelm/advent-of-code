use aoc::prelude::*;

fn mirror(s: &str) -> String {
    s.chars()
        .rev()
        .map(|c| b"10"["01".find(c).unwrap() as usize] as char)
        .collect()
}

fn checksum(s: &str) -> String {
    let s = s.as_bytes();
    let ret: String = (0..(s.len() / 2))
        .map(|i| if s[i * 2] != s[i * 2 + 1] { '0' } else { '1' })
        .collect();
    if ret.len() % 2 == 0 {
        checksum(&ret)
    } else {
        ret
    }
}

fn main() {
    let input = stdin_string();

    for n in [272, 35651584] {
        let mut fill = input.clone();
        while fill.len() < n {
            fill = format!("{fill}0{}", mirror(&fill));
        }
        println!("{}", checksum(&fill[..n]));
    }
}
