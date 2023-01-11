use aoc::prelude::*;
use std::{fmt, str::FromStr};

// Length of digit-handling chunk in code.
const CHUNK_LEN: usize = 18;
// Number of digits to process.
const N_DIGITS: usize = 14;
// Parameter vector extracted from code for jetting.
type Params = [[i64; 3]; N_DIGITS];

fn explode(n: i64) -> Vec<i64> {
    format!("{}", n)
        .chars()
        .rev()
        .map(|c| c.to_digit(10).unwrap() as i64)
        .collect()
}

fn implode(digits: &Vec<i64>) -> i64 {
    digits
        .iter()
        .rev()
        .map(|c| char::from_digit(*c as u32, 10).unwrap())
        .collect::<String>()
        .parse::<i64>()
        .unwrap()
}

#[derive(Copy, Clone, Debug)]
enum Var {
    X = 0,
    Y = 1,
    Z = 2,
    W = 3,
}

impl FromStr for Var {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "x" => Ok(Var::X),
            "y" => Ok(Var::Y),
            "z" => Ok(Var::Z),
            "w" => Ok(Var::W),
            _ => Err(()),
        }
    }
}

impl fmt::Display for Var {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Var::X => write!(f, "x"),
            Var::Y => write!(f, "y"),
            Var::Z => write!(f, "z"),
            Var::W => write!(f, "w"),
        }
    }
}

impl Var {
    fn eval(self, mem: &[i64; 4]) -> i64 {
        mem[self as usize]
    }
}

use Val::*;
#[derive(Copy, Clone, Debug)]
enum Val {
    Lit(i64),
    Ptr(Var),
}

impl FromStr for Val {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(n) = s.parse() {
            // Integer
            return Ok(Lit(n));
        } else {
            // Variable
            return Ok(Ptr(s.parse()?));
        }
    }
}

impl fmt::Display for Val {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Ptr(var) => write!(f, "{}", var),
            Lit(n) => write!(f, "{}", n),
        }
    }
}

impl Val {
    fn eval(self, mem: &[i64; 4]) -> i64 {
        match self {
            Lit(n) => n,
            Ptr(p) => p.eval(mem),
        }
    }
}

use Op::*;
#[derive(Copy, Clone, Debug)]
enum Op {
    Inp(Var),
    Add(Var, Val),
    Mul(Var, Val),
    Div(Var, Val),
    Mod(Var, Val),
    Eql(Var, Val),
}

impl FromStr for Op {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split(' ').collect::<Vec<&str>>().as_slice() {
            ["inp", var] => Ok(Inp(var.parse()?)),
            ["add", var, val] => Ok(Add(var.parse()?, val.parse()?)),
            ["mul", var, val] => Ok(Mul(var.parse()?, val.parse()?)),
            ["div", var, val] => Ok(Div(var.parse()?, val.parse()?)),
            ["mod", var, val] => Ok(Mod(var.parse()?, val.parse()?)),
            ["eql", var, val] => Ok(Eql(var.parse()?, val.parse()?)),
            _ => Err(()),
        }
    }
}

impl Op {
    fn eval(self, mem: &mut [i64; 4], input: &mut Vec<i64>) {
        match self {
            Inp(v) => mem[v as usize] = input.pop().expect("Out of input!"),
            Add(v, n) => mem[v as usize] = v.eval(mem) + n.eval(mem),
            Mul(v, n) => mem[v as usize] = v.eval(mem) * n.eval(mem),
            Div(v, n) => {
                debug_assert!(n.eval(mem) != 0);
                mem[v as usize] = v.eval(mem) / n.eval(mem);
            }
            Mod(v, n) => {
                debug_assert!(v.eval(mem) >= 0);
                debug_assert!(n.eval(mem) > 0);
                mem[v as usize] = v.eval(mem) % n.eval(mem);
            }
            Eql(v, n) => mem[v as usize] = (v.eval(mem) == n.eval(mem)) as i64,
        }
    }
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Inp(v) => write!(f, "inp {}", v),
            Add(v, n) => write!(f, "add {} {}", v, n),
            Mul(v, n) => write!(f, "mul {} {}", v, n),
            Div(v, n) => write!(f, "div {} {}", v, n),
            Mod(v, n) => write!(f, "mod {} {}", v, n),
            Eql(v, n) => write!(f, "eql {} {}", v, n),
        }
    }
}

/// Accelerated per-digit formula reverse-engineered from input.
fn jet(params: &Params, mut input: Vec<i64>) -> i64 {
    let mut z = 0;
    for [a, b, c] in params {
        // It's either the case that A = 1 and B > 10
        //   (s always 1, z multiplied by 26)
        // or A = 26 and B < 0
        //   (s can be zero, z shrunk to 1/26 IF w = z%26 + B)
        //
        // C is always positive, so w + C is always positive
        //   so s must be zero and z must shrink to zero for the last step to
        //   fit.
        //
        // Assume all shrinking steps must lock so that z reliably shrinks:
        //   WHEN A = 26, fix w = z%26 + B
        //
        // Growing digits are not obviously constrained.
        debug_assert!(*a == 26 || *a == 1);
        let w = input.pop().unwrap();
        let s = ((z % 26) + b != w) as i64;
        z = z / a * (25 * s + 1) + (w + c) * s;
    }
    z
}

/// Extract differing parameters from pattern picked up using
/// `per_digit_printout`.
fn extract_params(prog: &Vec<Op>) -> Result<Params, ()> {
    assert!(prog.len() == CHUNK_LEN * N_DIGITS);
    let mut ret: [[i64; 3]; 14] = Default::default();

    for i in 0..N_DIGITS {
        // Fuck yeah pattern matching
        let [Inp(Var::W),
             Mul(Var::X, Lit(0)),
             Add(Var::X, Ptr(Var::Z)),
             Mod(Var::X, Lit(26)),
             Div(Var::Z, Lit(a)),
             Add(Var::X, Lit(b)),
             Eql(Var::X, Ptr(Var::W)),
             Eql(Var::X, Lit(0)),
             Mul(Var::Y, Lit(0)),
             Add(Var::Y, Lit(25)),
             Mul(Var::Y, Ptr(Var::X)),
             Add(Var::Y, Lit(1)),
             Mul(Var::Z, Ptr(Var::Y)),
             Mul(Var::Y, Lit(0)),
             Add(Var::Y, Ptr(Var::W)),
             Add(Var::Y, Lit(c)),
             Mul(Var::Y, Ptr(Var::X)),
             Add(Var::Z, Ptr(Var::Y))] =
                 prog[i * CHUNK_LEN..(i + 1) * CHUNK_LEN] else {
             return Err(());
        };
        ret[i] = [a, b, c];
    }
    Ok(ret)
}

/// Reduce search space, force digits that make z register shrink.
fn force_number(params: &Params, n: i64) -> Option<i64> {
    let mut digits = explode(n);
    if digits.iter().any(|c| *c == 0) {
        return None;
    }

    if digits.len() != N_DIGITS / 2 {
        return None;
    }

    let mut ret = Vec::new();

    let mut z = 0;
    for [a, b, c] in params {
        let w;
        if *a == 1 {
            // Must expand z, digit is free.
            w = digits.pop().unwrap();
            z = z * 26 + w + c;
        } else {
            // Can shrink z, but only with a specific digit.
            debug_assert!(*a == 26);
            w = (z % 26) + b;
            if w < 1 || w > 9 {
                // And there might not be a valid digit.
                return None;
            }
            z = z / 26;
        }
        ret.push(w);
    }

    if z != 0 {
        return None;
    }

    ret = ret.into_iter().rev().collect();

    Some(implode(&ret))
}

fn main() {
    let prog: Vec<Op> = stdin_lines().map(|c| c.parse().unwrap()).collect();
    let params = extract_params(&prog).unwrap();

    for n in (1_000_000..10_000_000).rev() {
        if let Some(n) = force_number(&params, n) {
            assert_eq!(jet(&params, explode(n)), 0);
            println!("{}", n);
            break;
        }
    }

    for n in 1_000_000..10_000_000 {
        if let Some(n) = force_number(&params, n) {
            assert_eq!(jet(&params, explode(n)), 0);
            println!("{}", n);
            break;
        }
    }
}

/// Print out the mostly-same columns.
#[allow(dead_code)]
fn per_digit_printout(prog: &Vec<Op>) {
    let mut bins: [Vec<Op>; 14] = Default::default();
    let mut current_bin = 0;
    for &op in prog.iter().skip(1) {
        if matches!(op, Inp(_)) {
            current_bin += 1;
            continue;
        }
        bins[current_bin].push(op);
    }

    for row in 0..17 {
        for bin in &bins {
            let s = format!("{}", bin[row]);
            eprint!("{:<10}", s);
        }
        eprintln!();
    }
}

/// Reversed digits of descending zero-less fourteen digit numbers
#[allow(dead_code)]
fn model_numbers(params: &Params) -> impl Iterator<Item = Vec<i64>> + '_ {
    // If param 2 is >= 10, digit does not matter, so we can lock it to 9.
    fn is_live([_, b, _]: [i64; 3]) -> bool {
        b < 10
    }
    let live_digits =
        params.iter().rev().filter(|p| is_live(**p)).count() as u32;

    (10i64.pow(live_digits)..10i64.pow(live_digits + 1))
        .rev()
        .filter_map(move |n: i64| {
            let mut ret = Vec::new();
            let mut j = 0;
            for i in 0..14 {
                let digit;
                if is_live(params[13 - i]) {
                    digit = (n / 10i64.pow(j as u32)) % 10i64;
                    j += 1;
                } else {
                    digit = 9;
                }

                if digit == 0 {
                    return None;
                }
                ret.push(digit);
            }
            Some(ret)
        })
}

#[allow(dead_code)]
fn run(prog: &Vec<Op>, mut input: Vec<i64>) -> i64 {
    let mut mem = [0i64; 4];
    for op in prog {
        op.eval(&mut mem, &mut input);
    }
    mem[2]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn prog() -> Vec<Op> {
        include_str!("../../test/2124.txt")
            .lines()
            .map(|c| c.parse().unwrap())
            .collect()
    }

    fn params() -> Params {
        extract_params(&prog()).unwrap()
    }

    #[test]
    fn jet_matches_vm() {
        use rand::prelude::*;
        let mut rng = rand::thread_rng(); // lol nondeterminism

        let prog = prog();
        let params = params();

        for _ in 0..1000 {
            let n: Vec<i64> =
                (0..N_DIGITS).map(|_| rng.gen::<i64>() % 9 + 1).collect();
            assert_eq!(jet(&params, n.clone()), run(&prog, n.clone()));
        }
    }
}
