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

    fn eval(&self, x: f64) -> f64 {
        use Eqn::*;
        match self {
            X => x,
            N(n) => *n as f64,
            Op(op, a, b) => Operator(*op).apply(a.eval(x), b.eval(x)),
            _ => panic!("Can't eval"),
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

    // Human value becomes the unknown x.
    ops.insert("humn".to_string(), Eqn::X);

    // We get a linear equation, can just figure out the angle and offset to
    // find the zero point.

    let eq = Eqn::new_op('-', eval(&ops, &a), eval(&ops, &b));
    let f = |x| eq.eval(x);

    let dy = f(1.0) - f(0.0);
    let mut x = -f(0.0) / dy;

    // It's not going to be quite right for a big equation because of floating
    // point imprecision, so do some hill climbing.
    while f(x).abs() > 0.1 {
        x -= f(x) / dy;
    }

    let x = x.round() as i64;
    print!("{x}");
}
