use std::{collections::HashMap, fmt};

use aoc::prelude::*;

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
enum Eqn {
    X,
    N(i64),
    Op(char, Box<Eqn>, Box<Eqn>),
    S(String),
}

impl fmt::Display for Eqn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Eqn::*;
        match self {
            X => write!(f, "x"),
            N(n) => write!(f, "{n}"),
            Op(c, a, b) => write!(f, "({a} {c} {b})"),
            S(s) => write!(f, "{s}"),
        }
    }
}

impl Eqn {
    fn new_op(op: char, a: Eqn, b: Eqn) -> Eqn {
        use Eqn::*;
        match (a, b) {
            (N(a), N(b)) => N(Operator(op).apply(a, b)),
            (a, b) => Op(op, Box::new(a), Box::new(b)),
        }
    }

    fn s(&self) -> String {
        match self {
            Eqn::S(s) => s.clone(),
            _ => panic!("Not a string"),
        }
    }

    fn num(&self) -> i64 {
        match self {
            Eqn::N(n) => *n,
            _ => panic!("Not a number"),
        }
    }
}

fn eval(ops: &HashMap<String, Eqn>, id: &str) -> Eqn {
    use Eqn::*;
    if let Op(o, a, b) = &ops[id] {
        if let (S(a), S(b)) = (&**a, &**b) {
            return Eqn::new_op(*o, eval(ops, a), eval(ops, b));
        }
    }
    ops[id].clone()
}

fn main() {
    let mut ops: HashMap<String, Eqn> = HashMap::new();
    let num = re_parser::<(String, i64)>(r"^(.+): (\d+)$");
    let eqn =
        re_parser::<(String, String, char, String)>(r"^(.+): (.+) (.) (.+)$");

    for line in stdin_lines() {
        if let Ok((id, n)) = num(&line) {
            ops.insert(id, Eqn::N(n));
        } else if let Ok((id, a, op, b)) = eqn(&line) {
            ops.insert(
                id,
                Eqn::Op(op, Box::new(Eqn::S(a)), Box::new(Eqn::S(b))),
            );
        } else {
            panic!("bad line");
        }
    }

    // Break root monkey's equation apart, anticipating part 2.
    let Eqn::Op(op, a, b) = &ops["root"] else {
        panic!("Bad root");
    };
    let (a, b) = (a.s(), b.s());

    // Part 1

    println!(
        "{}",
        Operator(*op).apply(eval(&ops, &a).num(), eval(&ops, &b).num())
    );

    // Part 2

    // Human value becomes the unknown.
    ops.insert("humn".to_string(), Eqn::X);

    eprintln!("Copping out of actually solving p2. \
        Pls feed following to a symbolic algebra program and ask it to solve for x:");
    eprintln!("{} = {}", eval(&ops, &a), eval(&ops, &b));
}
