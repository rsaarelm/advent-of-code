use std::{collections::VecDeque, iter::IntoIterator};

use aoc::prelude::*;

#[derive(Copy, Clone)]
enum Op {
    Add(Option<u64>),
    Mul(Option<u64>),
}

impl Op {
    fn eval(self, x: u64) -> u64 {
        use Op::*;

        match self {
            Add(None) => x + x,
            Add(Some(a)) => x + a,
            Mul(None) => x * x,
            Mul(Some(a)) => x * a,
        }
    }
}

#[derive(Clone)]
struct Monkey {
    items: VecDeque<u64>,
    op: Op,
    divisor: u64,
    truth_monkey: usize,
    falsehood_monkey: usize,
}

impl Monkey {
    pub fn new<'a>(items: impl IntoIterator<Item = &'a u64>) -> Self {
        Monkey {
            items: items.into_iter().copied().collect(),
            op: Op::Add(None),
            divisor: 0,
            truth_monkey: 0,
            falsehood_monkey: 0,
        }
    }
}

fn compute_business_easy(mut monkeys: Vec<Monkey>, rounds: usize) -> u64 {
    let mut monkey_business = vec![0; monkeys.len()];

    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            while let Some(mut stress) = monkeys[i].items.pop_front() {
                monkey_business[i] += 1;
                stress = monkeys[i].op.eval(stress);
                // Easy mode, stress is divided.
                stress /= 3;
                let recipient = if stress % monkeys[i].divisor == 0 {
                    monkeys[i].truth_monkey
                } else {
                    monkeys[i].falsehood_monkey
                };
                monkeys[recipient].items.push_back(stress);
            }
        }
    }

    monkey_business.sort();
    monkey_business.reverse();
    monkey_business[0] * monkey_business[1]
}

fn compute_business(mut monkeys: Vec<Monkey>, rounds: usize) -> u64 {
    let mut monkey_business = vec![0; monkeys.len()];
    let modulo = monkeys.iter().map(|m| m.divisor).product::<u64>();

    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            while let Some(mut stress) = monkeys[i].items.pop_front() {
                monkey_business[i] += 1;
                stress = monkeys[i].op.eval(stress);
                let recipient = if stress % monkeys[i].divisor == 0 {
                    monkeys[i].truth_monkey
                } else {
                    monkeys[i].falsehood_monkey
                };
                stress %= modulo;
                monkeys[recipient].items.push_back(stress);
            }
        }
    }

    monkey_business.sort();
    monkey_business.reverse();
    monkey_business[0] * monkey_business[1]
}

fn main() {
    let mut monkeys = Vec::new();
    let mut monkey_idx = 0;

    let op_parser =
        re_parser::<(char, String)>("^  Operation: new = old (.) (.+)$");

    for line in stdin_lines() {
        if line.contains("Starting") {
            // Add a new monkey.
            monkeys.push(Monkey::new(&numbers(&line)));
            monkey_idx = monkeys.len() - 1;
        } else if let Ok((op, a)) = op_parser(&line) {
            // Represent the variable with None.
            let a: Option<u64> = a.parse().ok();
            let op = match op {
                '+' => Op::Add(a),
                '*' => Op::Mul(a),
                _ => panic!("Bad op"),
            };
            monkeys[monkey_idx].op = op;
        } else if line.contains("Test") {
            monkeys[monkey_idx].divisor = numbers(&line)[0];
        } else if line.contains("true") {
            monkeys[monkey_idx].truth_monkey = numbers(&line)[0];
        } else if line.contains("false") {
            monkeys[monkey_idx].falsehood_monkey = numbers(&line)[0];
        }
    }

    println!("{}", compute_business_easy(monkeys.clone(), 20));
    println!("{}", compute_business(monkeys, 10000));
}
