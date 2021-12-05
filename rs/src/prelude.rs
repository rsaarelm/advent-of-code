use lazy_static::lazy_static;
use regex::Regex;
use std::str::FromStr;

pub fn stdin_string() -> String {
    use std::{io, io::prelude::*};
    let mut ret = String::new();
    io::stdin().read_to_string(&mut ret).unwrap();
    ret
}

pub fn stdin_lines() -> impl Iterator<Item = String> + 'static {
    use std::io::{stdin, BufRead};
    std::iter::from_fn(|| stdin().lock().lines().next().map(|a| a.unwrap()))
}

/// Extract numbers from a string.
pub fn numbers<N: FromStr>(line: &str) -> Vec<N> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"-?\d+").unwrap();
    }

    RE.find_iter(line)
        .map(|s| {
            s.as_str()
                .parse()
                .unwrap_or_else(|_| panic!("Type didn't parse from integer"))
        })
        .collect()
}
