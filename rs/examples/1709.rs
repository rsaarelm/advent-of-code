use std::error::Error;

use aoc::prelude::*;

#[derive(Debug)]
enum Object {
    Junk(usize),
    Group(Vec<Object>),
}

use Object::*;

impl Object {
    pub fn score(&self, base: u32) -> u32 {
        match self {
            Junk(_) => 0,
            Group(seq) => {
                (base + 1) + seq.iter().map(|a| a.score(base + 1)).sum::<u32>()
            }
        }
    }

    pub fn junk_len(&self) -> usize {
        match self {
            Junk(n) => *n,
            Group(seq) => seq.iter().map(|a| a.junk_len()).sum::<usize>(),
        }
    }
}

impl FromStr for Object {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use nom::{
            branch::alt,
            bytes::complete::tag,
            character::complete::{anychar, none_of},
            combinator::map,
            multi::{many0, separated_list0},
            sequence::{delimited, preceded},
            IResult,
        };

        fn object(i: &str) -> IResult<&str, Object, ()> {
            let cancel =
                |i| map(preceded(tag("!"), anychar::<&str, ()>), |_| 0)(i);

            let junk = |i| {
                map(
                    delimited(
                        tag("<"),
                        many0(alt((cancel, map(none_of(">"), |_| 1)))),
                        tag(">"),
                    ),
                    |ns| Junk(ns.iter().sum::<usize>()),
                )(i)
            };

            let group = |i| {
                map(
                    delimited(
                        tag("{"),
                        separated_list0(tag(","), object),
                        tag("}"),
                    ),
                    Group,
                )(i)
            };

            // NB: Must use statement instead of expression here or borrow
            // checker gets cranky.
            return alt((junk, group))(i);
        }

        // XXX: Does not fail on trailing input.
        Ok(object(s)?.1)
    }
}

fn main() {
    let input: Object = from_stdin();
    println!("{}", input.score(0));
    println!("{}", input.junk_len());
}
