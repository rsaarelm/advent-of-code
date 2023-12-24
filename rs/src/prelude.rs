use std::{
    collections::{BTreeSet, VecDeque},
    convert::TryInto,
    fmt::{Debug, Write},
    hash::Hash,
    io::{stdin, BufRead},
};

use glam::Mat3;
use lazy_static::lazy_static;
use nalgebra::{DMatrix, DVector};
use num_traits::{One, Zero};
use regex::Regex;

pub use glam::{
    i64vec2, i64vec3, ivec2, ivec3, I64Vec2, I64Vec3, IVec2, IVec3,
    Vec3Swizzles,
};
pub use indexmap::IndexMap;
pub use memoize::memoize;
pub use si_trace_print::{efn, efx, en};
pub use std::str::FromStr;

// Faster hashmap and hashset implementations, no reason not to use these
// everywhere when you don't care about DDOS.
pub use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};

pub use crate::axis_box::{area, volume, Cube, Rect};
pub use crate::md5::md5sum;
pub use crate::ocr::{ocr, points, PointCloud};

pub const RIGHT: usize = 0;
pub const DOWN: usize = 1;
pub const LEFT: usize = 2;
pub const UP: usize = 3;

/// 3D Rotation matrices for the cube-spinning tasks.
///
/// Same order as DIR_4 contents, can be indexed with `LEFT`, `RIGHT`, `UP`
/// and `DOWN`.
#[rustfmt::skip]
pub const ROT_XY: [Mat3; 4] = [
    Mat3::from_cols_array(
        &[ 0.0,  0.0,  1.0,
           0.0,  1.0,  0.0,
          -1.0,  0.0,  0.0]),
    Mat3::from_cols_array(
        &[ 1.0,  0.0,  0.0,
           0.0,  0.0,  1.0,
           0.0, -1.0,  0.0]),
    Mat3::from_cols_array(
        &[ 0.0,  0.0, -1.0,
           0.0,  1.0,  0.0,
           1.0,  0.0,  0.0]),
    Mat3::from_cols_array(
        &[ 1.0,  0.0,  0.0,
           0.0,  0.0, -1.0,
           0.0,  1.0,  0.0]),
];

#[rustfmt::skip]
pub const ROT_CW: Mat3 =
    Mat3::from_cols_array(
        &[ 0.0,  1.0,  0.0,
          -1.0,  0.0,  0.0,
           0.0,  0.0,  1.0]);

#[rustfmt::skip]
pub const ROT_CCW: Mat3 =
    Mat3::from_cols_array(
        &[ 0.0, -1.0,  0.0,
           1.0,  0.0,  0.0,
           0.0,  0.0,  1.0]);

/// Can be indexed with `LEFT`, `RIGHT`, `UP` and `DOWN`.
pub const DIR_4: [IVec2; 4] =
    [ivec2(1, 0), ivec2(0, 1), ivec2(-1, 0), ivec2(0, -1)];

pub const DIR_8: [IVec2; 8] = [
    ivec2(1, 0),
    ivec2(1, 1),
    ivec2(0, 1),
    ivec2(-1, 1),
    ivec2(-1, 0),
    ivec2(-1, -1),
    ivec2(0, -1),
    ivec2(1, -1),
];

pub fn neighbors_4<T: Clone + Into<IVec2> + From<IVec2>>(
    p: T,
) -> impl Iterator<Item = T> {
    DIR_4.iter().map(move |&d| (d + p.clone().into()).into())
}

pub fn neighbors_8<T: Clone + Into<IVec2> + From<IVec2>>(
    p: T,
) -> impl Iterator<Item = T> {
    DIR_8.iter().map(move |&d| (d + p.clone().into()).into())
}

pub fn dir4(a: impl Into<IVec2>) -> usize {
    let a = a.into();
    DIR_4.iter().position(|&x| x == a).unwrap()
}

pub const SPACE_6: [IVec3; 6] = [
    ivec3(1, 0, 0),
    ivec3(-1, 0, 0),
    ivec3(0, 1, 0),
    ivec3(0, -1, 0),
    ivec3(0, 0, 1),
    ivec3(0, 0, -1),
];

pub fn stdin_string() -> String {
    use std::{io, io::prelude::*};
    let mut ret = String::new();
    io::stdin().read_to_string(&mut ret).unwrap();
    ret.truncate(ret.trim_end().len());
    ret
}

pub fn from_stdin<T>() -> T
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    stdin_string().parse().unwrap()
}

pub fn stdin_lines() -> impl Iterator<Item = String> + 'static {
    std::iter::from_fn(|| stdin().lock().lines().next().map(|a| a.unwrap()))
}

pub fn stdin_chars() -> impl Iterator<Item = char> + 'static {
    let s = stdin_string();
    let mut p = 0;
    std::iter::from_fn(move || match &s[p..].chars().next() {
        Some(c) => {
            p += c.len_utf8();
            Some(*c)
        }
        None => None,
    })
}

pub fn stdin_lines_as<T>() -> impl Iterator<Item = T> + 'static
where
    T: FromStr + Debug,
    <T as FromStr>::Err: Debug,
{
    std::iter::from_fn(|| {
        stdin()
            .lock()
            .lines()
            .next()
            .map(|a| a.unwrap().parse().unwrap())
    })
}

pub fn stdin_grid_iter() -> impl Iterator<Item = ([i32; 2], char)> {
    let mut x = 0;
    let mut y = 0;
    stdin_chars().filter_map(move |c| match c {
        '\n' => {
            x = 0;
            y += 1;
            None
        }
        c if c.is_whitespace() => {
            x += 1;
            None
        }
        c => {
            x += 1;
            Some(([x - 1, y], c))
        }
    })
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

pub fn to_array<T, const N: usize>(
    input: impl IntoIterator<Item = T>,
) -> [T; N] {
    input
        .into_iter()
        .collect::<Vec<T>>()
        .try_into()
        .unwrap_or_else(|_| panic!("fail"))
}

pub fn to_ivec2s(
    mut input: impl Iterator<Item = i32>,
) -> impl Iterator<Item = IVec2> {
    std::iter::from_fn(move || {
        let Some(x) = input.next() else {
            return None;
        };
        let Some(y) = input.next() else {
            return None;
        };
        Some(ivec2(x, y))
    })
}

pub fn hex_to_bytes(hex: impl AsRef<str>) -> Vec<u8> {
    hex.as_ref()
        .as_bytes()
        .chunks(2)
        .map(|c| {
            u8::from_str_radix(std::str::from_utf8(c).unwrap(), 16).unwrap()
        })
        .collect()
}

pub fn bytes_to_hex(digest: &[u8]) -> String {
    let mut ret = String::new();
    for b in digest {
        write!(&mut ret, "{b:02x}").unwrap();
    }
    ret
}

pub fn suffixes(s: &str) -> impl Iterator<Item = &str> {
    s.char_indices().map(|(n, _)| &s[n..])
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

/// Convert a string into a flattened rectangular array, return that and the
/// computed bounding box.
///
/// Will skip initial blank lines so that you can write multiline string
/// literals with indentation on the first line without having the double
/// quotes breaking up the first line. Trailing whitespace is omitted from
/// individual lines and from the input as a whole. Area beyond the lines is
/// filled with ASCII spaces `'\u{0020}'`.
///
/// ```
/// # use aoc::prelude::*;
///
/// let (range, grid) = grid("
///  a
/// bcd
///  e");
/// assert_eq!(range, area(3, 3));
/// ```
pub fn grid(s: impl AsRef<str>) -> (Rect<i32>, Vec<char>) {
    let s = s.as_ref().trim_end();
    let mut w = 0;
    let mut h = 0;
    let mut seen_content = false;
    let mut initial_blank_lines = 0;
    for line in s.lines() {
        if !seen_content && line.trim().is_empty() {
            initial_blank_lines += 1;
        } else {
            seen_content = true;
        }

        w = w.max(line.trim_end().len());
        h += 1;
    }
    let h = h - initial_blank_lines;
    let mut ret = vec![' '; w * h];
    let bounds = area(w as i32, h as i32);
    for (y, line) in s.lines().skip(initial_blank_lines).enumerate() {
        for (x, c) in line.trim_end().chars().enumerate() {
            ret[bounds.idx(ivec2(x as i32, y as i32))] = c;
        }
    }
    (bounds, ret)
}

pub fn stdin_grid() -> (Rect<i32>, Vec<char>) {
    grid(stdin_string())
}

pub fn print_grid(bounds: &Rect<i32>, mut f: impl FnMut(IVec2)) {
    for y in 0..bounds.height() {
        for x in 0..bounds.width() {
            f(ivec2(x, y) + IVec2::from(bounds.min()));
        }
        eprintln!();
    }
}

/// Generate a shortest paths map on a grid according to a neighbors function.
pub fn dijkstra_map<'a, T, I>(
    neighbors: impl Fn(&T) -> I + 'a,
    start: &T,
) -> impl Iterator<Item = (T, usize)> + 'a
where
    T: Clone + Eq + Hash + 'a,
    I: IntoIterator<Item = T>,
{
    let mut seen = HashSet::default();
    let mut edge = VecDeque::from([(start.clone(), 0)]);
    std::iter::from_fn(move || {
        // Candidates are in a queue and consumed first-in, first-out. This
        // should guarantee that the first time a node is popped from the queue
        // it shows the shortest path length from start to that node.

        while let Some((node, len)) = edge.pop_front() {
            if !seen.contains(&node) {
                seen.insert(node.clone());
                for n in neighbors(&node).into_iter() {
                    edge.push_back((n, len + 1));
                }
                return Some((node, len));
            }
        }
        None
    })
}

pub fn grid_astar<I>(
    start: &IVec2,
    end: &IVec2,
    neighbors: impl Fn(&IVec2) -> I,
) -> Option<Vec<IVec2>>
where
    I: IntoIterator<Item = IVec2>,
{
    astar_search(start, neighbors, |n| (*n - *end).chess_len(), |n| n == end)
}

/// Steer towards target using `heuristic` from `start`. Returns path
/// including both start and end positions. If heuristic never overestimates
/// the steps to reach goal, will return an optimal path.
pub fn astar_search<T, I, N>(
    start: &T,
    neighbors: impl Fn(&T) -> I,
    heuristic: impl Fn(&T) -> N,
    completed: impl Fn(&T) -> bool,
) -> Option<Vec<T>>
where
    T: Clone + Eq + Hash,
    N: Zero + One + Ord + Copy,
    I: IntoIterator<Item = T>,
{
    pathfinding::prelude::astar(
        start,
        |a| neighbors(a).into_iter().map(|c| (c, N::one())),
        heuristic,
        completed,
    )
    .map(|(path, _)| path)
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
///
/// let mut perm = vec![1u32, 2, 3, 4, 5, 6, 7, 8, 9];
/// next_prefix_permutation(&mut perm, 2);
/// assert_eq!(perm, vec![1u32, 3, 2, 4, 5, 6, 7, 8, 9]);
///
/// let mut perm = vec![1u32, 9, 2, 3];
/// next_prefix_permutation(&mut perm, 2);
/// assert_eq!(perm, vec![2u32, 1, 3, 9]);
/// ```
pub fn next_prefix_permutation(
    perm: &mut [impl Ord],
    prefix_len: usize,
) -> bool {
    let prefix_len = prefix_len.min(perm.len());
    if prefix_len < 1 || perm.len() < 2 {
        return false;
    }
    let i = prefix_len - 1;

    if let Some((j, _)) = perm
        .iter()
        .enumerate()
        .filter(|&(j, n)| j > i && *n > perm[i])
        .min()
    {
        if j > i {
            perm.swap(i, j);
            perm[i + 1..].sort();
            return true;
        }
    }
    perm[i + 1..].sort_by(|a, b| b.cmp(a));
    next_permutation(perm)
}

/// Produce a histogram of repeated occurrences in input, sorted by most
/// common element first and by element value on ties.
pub fn histogram<T: Clone + Eq + Hash + Ord>(
    input: impl IntoIterator<Item = T>,
) -> impl Iterator<Item = (T, usize)> {
    let mut hist: HashMap<T, usize> = HashMap::default();

    for i in input.into_iter() {
        *hist.entry(i).or_default() += 1;
    }

    let mut hist: Vec<(T, usize)> = hist.into_iter().collect();
    hist.sort_by_key(|(t, n)| (usize::MAX - *n, t.clone()));
    hist.into_iter()
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
tuple_parseable!(T1, T2, T3, T4, T5, T6; 1, 2, 3, 4, 5, 6);
tuple_parseable!(T1, T2, T3, T4, T5, T6, T7; 1, 2, 3, 4, 5, 6, 7);

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

/// Convenience wrapper for arithmetic interpreters.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Operator(pub char);

impl Operator {
    pub fn apply<
        T: std::ops::Add<Output = T>
            + std::ops::Sub<Output = T>
            + std::ops::Mul<Output = T>
            + std::ops::Div<Output = T>,
    >(
        self,
        a: T,
        b: T,
    ) -> T {
        match self.0 {
            '+' => a + b,
            '-' => a - b,
            '*' => a * b,
            '/' => a / b,
            _ => panic!("Unknown operator"),
        }
    }
}

/// Return corresponding comparison function given a comparison operator
/// string.
pub fn cmp_fn(op: &str) -> fn(i32, i32) -> bool {
    match op {
        "==" => |a, b| a == b,
        "!=" => |a, b| a != b,
        "<=" => |a, b| a <= b,
        "<" => |a, b| a < b,
        ">=" => |a, b| a >= b,
        ">" => |a, b| a > b,
        _ => panic!("cmp: Unknown op {op}"),
    }
}

// XXX: These are expensive, allocating a full new array. This stuff could be
// done cheaply by introducing some kind of intermediate interface on arrays
// that can apply transformation functions to the access coordinates, but that
// would introduce additional design complexities.

/// Create a new grid that's the original mirrored along the Y-axis.
pub fn mirror_grid<T: Clone>(
    (bounds, buf): (Rect<i32>, Vec<T>),
) -> (Rect<i32>, Vec<T>) {
    let w = bounds.width() - 1;
    let ret_buf = (0..buf.len())
        .map(|i| {
            let [x, y] = bounds.get(i);
            buf[bounds.idx([w - x, y])].clone()
        })
        .collect();

    (bounds, ret_buf)
}

/// Create a new grid that's the original rotated 90 degrees clockwise.
pub fn rotate_grid<T: Clone>(
    (bounds, buf): (Rect<i32>, Vec<T>),
) -> (Rect<i32>, Vec<T>) {
    let [x, y] = bounds.max();
    let ret_bounds = Rect::new(bounds.min(), [y, x]);
    let h = bounds.height() - 1;
    let ret_buf = (0..buf.len())
        .map(|i| {
            let [x, y] = ret_bounds.get(i);
            buf[bounds.idx([y, h - x])].clone()
        })
        .collect();

    (ret_bounds, ret_buf)
}

pub trait VecExt {
    /// Vector length in taxicab metric.
    fn taxi_len(self) -> i32;
    /// Vector length in chessboard metric.
    fn chess_len(self) -> i32;
}

impl VecExt for IVec2 {
    fn taxi_len(self) -> i32 {
        self.x.abs() + self.y.abs()
    }

    fn chess_len(self) -> i32 {
        self.x.abs().max(self.y.abs())
    }
}

impl VecExt for IVec3 {
    fn taxi_len(self) -> i32 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }

    fn chess_len(self) -> i32 {
        self.x.abs().max(self.y.abs()).max(self.z.abs())
    }
}

pub trait Rotate {
    /// Rotate clockwise
    fn cw(self) -> Self;
    /// Rotate counterclockwise.
    fn ccw(self) -> Self;
}

impl Rotate for IVec2 {
    fn cw(self) -> Self {
        ivec2(-self.y, self.x)
    }

    fn ccw(self) -> Self {
        ivec2(self.y, -self.x)
    }
}

impl Rotate for I64Vec2 {
    fn cw(self) -> Self {
        i64vec2(-self.y, self.x)
    }

    fn ccw(self) -> Self {
        i64vec2(self.y, -self.x)
    }
}

/// Map register to index.
pub fn reg(c: char) -> usize {
    (c as u8 - b'a') as usize
}

pub fn is_prime<N>(n: N) -> bool
where
    N: Copy
        + Zero
        + One
        + std::ops::Add
        + std::ops::Rem<Output = N>
        + std::ops::Mul
        + PartialOrd
        + PartialEq,
{
    let zero = N::zero();
    let mut i = N::one() + N::one();
    loop {
        if i * i > n {
            break;
        }

        if n % i == zero {
            return false;
        }

        i = i + N::one();
    }

    true
}

/// Polygon area using the shoelace formula.
pub fn polygon_area(vertices: &[I64Vec2]) -> i64 {
    // Determinant
    fn det(a: i64, b: i64, c: i64, d: i64) -> i64 {
        i64vec2(a, b).perp_dot(i64vec2(c, d))
    }

    let mut area2: i64 = 0;
    for i in 0..vertices.len() {
        let a = vertices[i];
        let b = vertices[(i + 1) % vertices.len()];
        area2 += det(a.x, b.x, a.y, b.y);
    }
    area2 / 2
}

/// Find the `xs.len() - 1` degree polynomial that fits the input points.
pub fn fit_polynomial(xs: &[f64], ys: &[f64]) -> Vec<f64> {
    assert_eq!(xs.len(), ys.len());
    assert!(xs.len() >= 2);

    // | x_1^0  x_1^1  x_2^2  ... |
    // | x_2^0  x_2^1  x_2^2  ... |
    // | ...                      |
    let a = DMatrix::from_fn(xs.len(), xs.len(), |i, j| xs[i].powi(j as i32));

    let b = DVector::from_row_slice(ys);
    let decomp = a.svd(true, true);

    decomp
        .solve(&b, 1.0e-12)
        .expect("Failed to fit polynomial")
        .data
        .into()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_numbers() {
        let v: Vec<i32> = vec![1, 2, 3, 4];
        let s: Vec<i32> = numbers("1 2 3 4");
        assert_eq!(s, v);

        let a: [i32; 4] = [1, 2, 3, 4];
        let s: [i32; 4] = fixed_numbers("1, 2, 3, 4");
        assert_eq!(a, s);
    }

    #[test]
    fn permutations() {
        let mut perm: Vec<u32> = (0..10).collect();

        let mut n = 0;
        loop {
            n += 1;
            let p1: Vec<u32> = perm[..6].to_vec();

            if !next_prefix_permutation(&mut perm, 6) {
                break;
            }

            let p2: Vec<u32> = perm[..6].to_vec();
            assert_ne!(p1, p2);
            assert!(p2 > p1);
        }
        assert_eq!(n, 151_200);
    }

    #[test]
    fn rotations() {
        use glam::vec3;
        assert_eq!(ROT_XY[UP] * vec3(-1.0, -1.0, -1.0), vec3(-1.0, -1.0, 1.0));
        assert_eq!(
            ROT_XY[DOWN] * vec3(-1.0, -1.0, -1.0),
            vec3(-1.0, 1.0, -1.0)
        );
        assert_eq!(
            ROT_XY[LEFT] * vec3(-1.0, -1.0, -1.0),
            vec3(-1.0, -1.0, 1.0)
        );
        assert_eq!(
            ROT_XY[RIGHT] * vec3(-1.0, -1.0, -1.0),
            vec3(1.0, -1.0, -1.0)
        );

        assert_eq!(ROT_CW * vec3(1.0, 0.0, 0.0), vec3(0.0, 1.0, 0.0));
        assert_eq!(ROT_CCW * vec3(1.0, 0.0, 0.0), vec3(0.0, -1.0, 0.0));
    }
}
