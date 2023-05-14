use std::error::Error;

use aoc::prelude::*;

#[derive(Copy, Clone, Debug)]
enum Move {
    Spin(usize),
    Exchange(usize, usize),
    Partner(u8, u8),
}

use Move::*;

impl FromStr for Move {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use nom::{
            branch::alt,
            bytes::complete::tag,
            character::complete::{anychar, digit1},
            combinator::map,
            sequence::{preceded, separated_pair},
            IResult,
        };

        fn item(i: &str) -> IResult<&str, Move, ()> {
            alt((
                map(preceded(tag("s"), digit1), |n: &str| {
                    Spin(n.parse().unwrap())
                }),
                map(
                    preceded(
                        tag("x"),
                        separated_pair(digit1, tag("/"), digit1),
                    ),
                    |(a, b): (&str, &str)| {
                        Exchange(a.parse().unwrap(), b.parse().unwrap())
                    },
                ),
                map(
                    preceded(
                        tag("p"),
                        separated_pair(anychar, tag("/"), anychar),
                    ),
                    |(a, b): (char, char)| Partner(a as u8, b as u8),
                ),
            ))(i)
        }

        Ok(item(s)?.1)
    }
}

impl Move {
    pub fn apply(&self, state: &mut [u8]) {
        match self {
            Spin(n) => state.rotate_right(*n),
            Exchange(x, y) => (state[*y], state[*x]) = (state[*x], state[*y]),
            Partner(a, b) => {
                let x = state.iter().position(|i| i == a).unwrap();
                let y = state.iter().position(|i| i == b).unwrap();
                (state[y], state[x]) = (state[x], state[y]);
            }
        }
    }
}

fn main() {
    let input: Vec<Move> = stdin_string()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();
    let mut state: Vec<u8> = (0..16).map(|n| b'a' + n).collect();

    for m in &input {
        m.apply(&mut state);
    }

    println!("{}", std::str::from_utf8(&state).unwrap());

    // P2
    let mut state: Vec<u8> = (0..16).map(|n| b'a' + n).collect();

    // Find the cycle.
    let mut seen = HashMap::default();

    for i in 0.. {
        if let Some(idx) = seen.get(&state) {
            // Cycle found.
            // XXX: Be lazy and don't cover the case where it doesn't loop
            // back to the very beginning.
            assert_eq!(*idx, 0);
            break;
        }

        seen.insert(state.clone(), i);

        for m in &input {
            m.apply(&mut state);
        }
    }

    let offset = 1_000_000_000 % seen.len();
    let state = seen.iter().find_map(|(k, v)| (*v == offset).then_some(k)).unwrap();
    println!("{}", std::str::from_utf8(state).unwrap());
}
