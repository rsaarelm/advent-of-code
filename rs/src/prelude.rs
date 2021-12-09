use lazy_static::lazy_static;
pub use memoize::memoize;
use regex::Regex;
use std::{collections::{HashSet, BTreeSet}, convert::TryInto, hash::Hash, str::FromStr};

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

lazy_static! {
    static ref SIGNED_INTEGER: Regex = Regex::new(r"-?\d+").unwrap();
}

/// Extract numbers from a string.
pub fn numbers<T: FromStr>(line: impl AsRef<str>) -> Vec<T> {
    SIGNED_INTEGER
        .find_iter(line.as_ref())
        .map(|s| {
            s.as_str()
                .parse()
                .unwrap_or_else(|_| panic!("Type didn't parse from integer"))
        })
        .collect()
}

/// Extract a fixed amount of numbers from a string into an array.
pub fn fixed_numbers<T, const N: usize>(line: impl AsRef<str>) -> [T; N]
where
    T: Copy + FromStr,
{
    let elts: Vec<T> = SIGNED_INTEGER
        .find_iter(line.as_ref())
        .map(|s| {
            s.as_str()
                .parse()
                .unwrap_or_else(|_| panic!("Type didn't parse from integer"))
        })
        .collect();

    elts.as_slice().try_into().unwrap()
}

pub trait Row: Sized {
    fn parse(s: impl AsRef<str>) -> Self;
}

impl<T: FromStr> Row for Vec<T> {
    fn parse(s: impl AsRef<str>) -> Self {
        numbers(s)
    }
}

impl<T: Copy + FromStr, const N: usize> Row for [T; N] {
    fn parse(s: impl AsRef<str>) -> Self {
        fixed_numbers(s)
    }
}

pub trait Matrix: Sized {
    fn parse(s: impl AsRef<str>) -> Self;
}

impl<T: Row> Matrix for Vec<T> {
    fn parse(s: impl AsRef<str>) -> Self {
        s.as_ref()
            .trim()
            .split('\n')
            .map(|line| Row::parse(line))
            .collect()
    }
}

impl<T: Row + Clone, const N: usize> Matrix for [T; N] {
    fn parse(s: impl AsRef<str>) -> Self {
        let mut ret: [T; N] = unsafe { std::mem::zeroed() };
        let elts: Vec<T> = s
            .as_ref()
            .trim()
            .split('\n')
            .map(|line| Row::parse(line))
            .collect();
        ret.clone_from_slice(elts.as_slice());
        ret
    }
}

// Implement the missing pop method for sets.
pub trait SetUtil {
    type Item;
    fn pop(&mut self) -> Option<Self::Item>;
}

impl<N: Hash + Eq + Clone> SetUtil for HashSet<N> {
    type Item = N;

    fn pop(&mut self) -> Option<Self::Item> {
        if let Some(elt) = self.iter().next().cloned() {
            self.remove(&elt);
            Some(elt)
        } else {
            None
        }
    }
}

impl<N: Ord + Eq + Clone> SetUtil for BTreeSet<N> {
    type Item = N;

    fn pop(&mut self) -> Option<Self::Item> {
        if let Some(elt) = self.iter().next().cloned() {
            self.remove(&elt);
            Some(elt)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_numbers() {
        let v: Vec<i32> = vec![1, 2, 3, 4];
        let s: Vec<i32> = numbers("1 2 3 4");
        assert_eq!(s, v);

        let a: [i32; 4] = [1, 2, 3, 4];
        let s: [i32; 4] = fixed_numbers("1, 2, 3, 4");
        assert_eq!(a, s);
    }
}
