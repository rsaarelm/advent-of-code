use aoc::prelude::*;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    sequence::{preceded, separated_pair},
    Finish, Parser,
};

enum Term {
    Int(u16),
    Sym(String),
}

impl Term {
    fn eval(
        &self,
        s: &HashMap<String, Op>,
        m: &mut HashMap<String, u16>,
    ) -> u16 {
        match self {
            Int(n) => *n,
            Sym(x) => eval(s, m, x),
        }
    }
}

enum Op {
    Id(Term),
    Not(Term),
    Or(Term, Term),
    And(Term, Term),
    Lshift(Term, u16),
    Rshift(Term, u16),
}

use Op::*;
use Term::*;

fn eval(
    s: &HashMap<String, Op>,
    m: &mut HashMap<String, u16>,
    sym: &str,
) -> u16 {
    if let Some(&n) = m.get(sym) {
        return n;
    }
    let ret = match &s[sym] {
        Id(t) => t.eval(s, m),
        Not(t) => !t.eval(s, m),
        Or(t, u) => t.eval(s, m) | u.eval(s, m),
        And(t, u) => t.eval(s, m) & u.eval(s, m),
        Lshift(t, n) => t.eval(s, m) << n,
        Rshift(t, n) => t.eval(s, m) >> n,
    };
    m.insert(sym.to_string(), ret);
    ret
}

fn main() {
    let num = || {
        take_while1::<_, _, ()>(|c: char| c.is_numeric())
            .map(|s: &str| s.parse::<u16>().unwrap())
    };

    let term = || {
        alt((
            num().map(Int),
            take_while1(|c: char| c.is_alphabetic())
                .map(|s: &str| Sym(s.to_string())),
        ))
    };

    let op = || {
        alt::<_, _, (), _>((
            preceded(tag("NOT "), term()).map(Not),
            separated_pair(term(), tag(" OR "), term()).map(|(a, b)| Or(a, b)),
            separated_pair(term(), tag(" AND "), term())
                .map(|(a, b)| And(a, b)),
            separated_pair(term(), tag(" LSHIFT "), num())
                .map(|(a, b)| Lshift(a, b)),
            separated_pair(term(), tag(" RSHIFT "), num())
                .map(|(a, b)| Rshift(a, b)),
            term().map(Id),
        ))
    };

    let mut wires = HashMap::default();

    let input = stdin_string();
    for line in input.lines() {
        match separated_pair(op(), tag(" -> "), term())(line).finish() {
            Ok((_, (o, Sym(s)))) => {
                wires.insert(s.to_string(), o);
            }
            _ => panic!("Bad input"),
        }
    }

    // Part 1

    let a1 = eval(&wires, &mut HashMap::default(), "a");
    println!("{a1}");

    // Part 2

    wires.insert("b".to_string(), Id(Int(a1)));
    let a2 = eval(&wires, &mut HashMap::default(), "a");
    println!("{a2}");
}
