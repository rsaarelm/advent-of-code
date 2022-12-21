use std::{collections::HashMap, str::FromStr};

use aoc::prelude::*;
use lazy_static::lazy_static;
use regex::Regex;

type F = fraction::Fraction;

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
enum Eqn<T> {
    N(i64),
    Op(char, T, T),
}

use Eqn::*;

impl<T> Eqn<T> {
    fn num(&self) -> Option<i64> {
        if let &N(n) = self {
            Some(n)
        } else {
            None
        }
    }

    fn map<U>(self, f: impl Fn(T) -> U) -> Eqn<U> {
        match self {
            N(n) => N(n),
            Op(op, a, b) => Op(op, f(a), f(b)),
        }
    }
}

lazy_static! {
    static ref EQN: Regex = Regex::new("^(.+) (.) (.+)$").unwrap();
}

impl<T: FromStr> FromStr for Eqn<T> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(n) = s.parse::<i64>() {
            Ok(N(n))
        } else {
            let (a, op, b) = <(T, char, T) as RegexParseable>::parse(&EQN, s)?;
            Ok(Op(op, a, b))
        }
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
enum Term {
    X,
    E(Box<Eqn<Term>>),
}

use Term::*;

impl Term {
    fn eval(&self, x: impl Into<F>) -> F {
        match self {
            X => x.into(),
            E(eqn) => eqn.eval(x),
        }
    }
}

impl Eqn<Term> {
    fn eval(&self, x: impl Into<F>) -> F {
        let x = x.into();
        match self {
            N(n) => F::from(*n),
            Op(op, a, b) => Operator(*op).apply(a.eval(x), b.eval(x)),
        }
    }

    fn reduce(&self) -> Self {
        match self {
            Op(o, E(a), E(b)) => {
                let (a, b) = (a.reduce(), b.reduce());

                match (a, b) {
                    (N(a), N(b)) => N(Operator(*o).apply(a, b)),
                    (a, b) => Op(*o, E(Box::new(a)), E(Box::new(b))),
                }
            }
            x => x.clone(),
        }
    }
}

fn resolve(raw: &HashMap<String, Eqn<String>>, var: &str) -> Eqn<Term> {
    raw[var].clone().map(|t| {
        if !raw.contains_key(&t) {
            X
        } else {
            E(Box::new(resolve(raw, &t).reduce()))
        }
    })
}

fn main() {
    let mut eqns: HashMap<String, Eqn<String>> =
        parsed_stdin_lines(r"^(.+): (.+)$").collect();

    // Part 1

    let p1_x: i64 = eqns["humn"].num().unwrap();
    eqns.remove("humn");

    let Op(op, a, b) = resolve(&eqns, "root") else {
        panic!("Bad root")
    };

    println!("{}", Operator(op).apply(a.eval(p1_x), b.eval(p1_x)));

    // Part 2

    // Result is a linear function, use fraction type for precision and solve
    // x in closed form.
    let eq = Op('-', a, b);
    let f = |x| eq.eval(x);

    // Solve ax + b = 0 for x
    // a = f(1) - f(0)
    // b = f(0)
    // x = -b / a = -f(0) / (f(1) - f(0))
    println!("{}", -f(0) / (f(1) - f(0)));
}
