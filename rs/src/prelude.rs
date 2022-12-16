use std::{
    collections::{BTreeSet, HashSet, VecDeque},
    convert::TryInto,
    fmt::Debug,
    hash::Hash,
    str::FromStr,
};

use lazy_static::lazy_static;
pub use memoize::memoize;
use regex::Regex;
use rustc_hash::FxHashSet;

pub type Vec2 = euclid::default::Vector2D<i64>;
pub type Rect = euclid::default::Rect<i64>;
pub use euclid::vec2;

pub const DIR_4: [Vec2; 4] = [vec2(1, 0), vec2(0, 1), vec2(-1, 0), vec2(0, -1)];

pub const DIR_8: [Vec2; 8] = [
    vec2(1, 0),
    vec2(1, 1),
    vec2(0, 1),
    vec2(-1, 1),
    vec2(-1, 0),
    vec2(-1, -1),
    vec2(0, -1),
    vec2(1, -1),
];

pub fn stdin_string() -> String {
    use std::{io, io::prelude::*};
    let mut ret = String::new();
    io::stdin().read_to_string(&mut ret).unwrap();
    ret.truncate(ret.trim_end().len());
    ret
}

pub fn stdin_lines() -> impl Iterator<Item = String> + 'static {
    use std::io::{stdin, BufRead};
    std::iter::from_fn(|| stdin().lock().lines().next().map(|a| a.unwrap()))
}

pub fn stdin_lines_as<T>() -> impl Iterator<Item = T> + 'static
where
    T: FromStr + Debug,
    <T as FromStr>::Err: Debug,
{
    use std::io::{stdin, BufRead};
    std::iter::from_fn(|| {
        stdin()
            .lock()
            .lines()
            .next()
            .map(|a| a.unwrap().parse().unwrap())
    })
}

pub fn stdin_grid() -> (usize, usize, Vec<Vec<char>>) {
    let mut grid: Vec<Vec<char>> = stdin_lines()
        .filter_map(|line| {
            let line = line.trim_end();
            if !line.is_empty() {
                Some(line.chars().collect())
            } else {
                None
            }
        })
        .collect();
    let w = grid.iter().map(|line| line.len()).max().unwrap_or(0);
    let h = grid.len();

    // Make sure the right edge is uniform.
    for line in grid.iter_mut() {
        while line.len() < w {
            line.push(' ');
        }
    }

    (w, h, grid)
}

pub fn stdin_grid_into<T: From<char>>() -> (usize, usize, Vec<Vec<T>>) {
    let (w, h, grid) = stdin_grid();

    (
        w,
        h,
        grid.into_iter()
            .map(|line| line.into_iter().map(T::from).collect())
            .collect(),
    )
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

pub fn to_vec2s(mut input: impl Iterator<Item = i64>) -> impl Iterator<Item = Vec2> {
    std::iter::from_fn(move || {
        let Some(x) = input.next() else { return None; };
        let Some(y) = input.next() else { return None; };
        Some(vec2(x, y))
    })
}

pub fn hex_to_bytes(hex: impl AsRef<str>) -> Vec<u8> {
    hex.as_ref()
        .as_bytes()
        .chunks(2)
        .map(|c| u8::from_str_radix(std::str::from_utf8(c).unwrap(), 16).unwrap())
        .collect()
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

pub trait Grid {
    type Item;
    fn get(&self, pos: Vec2) -> Self::Item;
    fn dim(&self) -> Vec2 {
        // Default to infinite grid with no meaningful dim value.
        vec2(-1, -1)
    }

    fn contains(&self, pos: Vec2) -> bool {
        let dim = self.dim();
        // Magic value for infinite grid.
        if dim == vec2(-1, -1) {
            true
        } else {
            pos.x >= 0 && pos.x < dim.x && pos.y >= 0 && pos.y < dim.y
        }
    }
}

impl<T: Clone> Grid for Vec<Vec<T>> {
    type Item = T;

    fn get(&self, pos: Vec2) -> Self::Item {
        self[pos.y as usize][pos.x as usize].clone()
    }

    fn dim(&self) -> Vec2 {
        if self.is_empty() {
            vec2(0, 0)
        } else {
            vec2(self[0].len() as i64, self.len() as i64)
        }
    }
}

pub struct InfiniteGrid<G>(G);

impl<T: Default + Clone, G: Grid<Item = T>> Grid for InfiniteGrid<G> {
    type Item = T;

    fn get(&self, pos: Vec2) -> Self::Item {
        if self.0.contains(pos) {
            self.0.get(pos)
        } else {
            T::default()
        }
    }
}

/// Generate a shortest paths map on a grid according to a neighbors function.
pub fn dijkstra_map<'a, T, I>(
    neighbors: impl Fn(&T) -> I + 'a,
    start: T,
) -> impl Iterator<Item = (T, usize)> + 'a
where
    T: Clone + Eq + Hash + 'a,
    I: Iterator<Item = T>,
{
    let mut seen = FxHashSet::default();
    let mut edge = VecDeque::from([(start, 0)]);
    std::iter::from_fn(move || {
        // Candidates are in a queue and consumed first-in, first-out. This
        // should guarantee that the first time a node is popped from the queue
        // it shows the shortest path length from start to that node.

        while let Some((node, len)) = edge.pop_front() {
            if !seen.contains(&node) {
                seen.insert(node.clone());
                for n in neighbors(&node) {
                    edge.push_back((n, len + 1));
                }
                return Some((node, len));
            }
        }
        None
    })
}

/// Try to advance slice to next lexical permutation.
///
/// Returns false if the slice is the last permutation.
///
/// ```
/// use aoc::prelude::*;
///
/// let mut perm = vec![1u32, 2, 3];
/// next_permutation(&mut perm);
/// assert_eq!(perm, vec![1u32, 3, 2]);
/// ```
pub fn next_permutation(perm: &mut [impl Ord]) -> bool {
    if perm.len() < 2 {
        return false;
    }

    for i in (0..(perm.len() - 1)).rev() {
        if perm[i] < perm[i + 1] {
            let (j, _) = perm
                .iter()
                .enumerate()
                .rev()
                .find(|(_, k)| **k > perm[i])
                .unwrap();
            perm.swap(i, j);
            perm[i + 1..].reverse();
            return true;
        }
    }

    false
}

/// Advance slice to the next lexical permutation where the value of
/// `perm[0..prefix_len]` changes, skipping over any permutations past
/// `prefix_len`.
///
/// Returns false if `perm[0..prefix_len]` is the last permutation.
///
/// ```
/// use aoc::prelude::*;
///
/// let mut perm = vec![1u32, 4, 2, 8];
/// next_prefix_permutation(&mut perm, 2);
/// assert_eq!(perm, vec![1u32, 8, 2, 4]);
///
/// let mut perm = vec![1u32, 4, 3, 2];
/// next_prefix_permutation(&mut perm, 2);
/// assert_eq!(perm, vec![2u32, 1, 3, 4]);
/// ```
pub fn next_prefix_permutation(perm: &mut [impl Ord], prefix_len: usize) -> bool {
    let prefix_len = prefix_len.min(perm.len());
    if prefix_len < 1 || perm.len() < 2 {
        return false;
    }
    let i = prefix_len - 1;

    if let Some((j, _)) = perm.iter().enumerate().rev().find(|(_, k)| **k > perm[i]) {
        if j > i {
            perm.swap(i, j);
            perm[i + 1..].sort();
            return true;
        }
    }
    next_permutation(perm)
}

pub trait RegexParseable: Sized {
    type Error;

    fn parse(re: &Regex, input: &str) -> Result<Self, Self::Error>;
}

// Construct generic parseability for heterogeneous tuples up to however many
// elements we expect to show up in the assignments.

macro_rules! tuple_parseable {
    ($($t:ident),+; $($n:expr),+) => {
        impl<$($t),+> RegexParseable for ($($t,)+)
        where
            $($t: std::str::FromStr),+
        {
            type Error = ();

            fn parse(re: &Regex, input: &str) -> Result<Self, Self::Error> {
                let caps = re.captures(input).ok_or(())?;

                Ok((
                    $(caps.get($n).ok_or(())?.as_str().parse().map_err(|_| ())?,)+
                ))
            }
        }
    };
}

// XXX: Can't implement for generic T: FromStr since that would clash with the
// tuple_parseable defs.

macro_rules! primitive_parseable {
    ($($t:ty),+) => {
        $(impl RegexParseable for $t {
            type Error = ();

            fn parse(re: &Regex, input: &str) -> Result<Self, Self::Error> {
                let caps = re.captures(input).ok_or(())?;
                caps.get(1).ok_or(())?.as_str().parse().map_err(|_| ())
            }
        })+
    }
}

tuple_parseable!(T1; 1);
tuple_parseable!(T1, T2; 1, 2);
tuple_parseable!(T1, T2, T3; 1, 2, 3);
tuple_parseable!(T1, T2, T3, T4; 1, 2, 3, 4);
tuple_parseable!(T1, T2, T3, T4, T5; 1, 2, 3, 4, 5);

primitive_parseable!(
    String, char, //
    u8, u16, u32, u64, u128, usize, //
    i8, i16, i32, i64, i128, isize
);

// Concrete regex parser that can be stored in lazy_static.
pub struct ReParser<T> {
    re: Regex,
    marker: std::marker::PhantomData<T>,
}

impl<T: RegexParseable> ReParser<T> {
    pub fn new(re: &str) -> Self {
        ReParser {
            re: Regex::new(re).expect("Failed to construct regular expression"),
            marker: Default::default(),
        }
    }

    pub fn parse(&self, s: &str) -> Result<T, <T as RegexParseable>::Error> {
        T::parse(&self.re, s)
    }
}

// Closurized regex parser.
pub fn re_parser<T: RegexParseable>(
    re: &str,
) -> impl Fn(&str) -> Result<T, <T as RegexParseable>::Error> {
    // Build a closure so we can reuse the expensive-to-construct regex.
    let re = Regex::new(re).expect("Failed to construct regular expression");

    move |s: &str| T::parse(&re, s)
}

/// Parse all stdin lines using the parsing regex.
pub fn parsed_stdin_lines<T>(re: &str) -> impl Iterator<Item = T>
where
    T: RegexParseable + 'static,
{
    let parser = re_parser(re);
    stdin_lines().map(move |line| {
        parser(&line)
            .map_err(|_| ())
            .expect("Failed to parse input line")
    })
}

pub trait VecExt {
    type Element;

    fn signum(self) -> Self;
    fn max_element(self) -> Self::Element;
    fn taxicab_len(self) -> Self::Element;
}

impl VecExt for Vec2 {
    type Element = i64;

    fn signum(self) -> Self {
        vec2(self.x.signum(), self.y.signum())
    }

    fn max_element(self) -> Self::Element {
        self.x.max(self.y)
    }

    fn taxicab_len(self) -> Self::Element {
        self.x.abs() + self.y.abs()
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
