use aoc::prelude::*;
use nom::{
    branch::alt,
    bytes::complete::take_while_m_n,
    character::complete::{char, satisfy},
    combinator::{map, map_res},
    multi::many0,
    sequence::{delimited, preceded},
};

fn parse(s: &str) -> String {
    fn is_hex_digit(c: char) -> bool {
        c.is_ascii_hexdigit()
    }

    fn from_hex(i: &str) -> Result<char, std::num::ParseIntError> {
        u8::from_str_radix(i, 16).map(|n| n as char)
    }

    let encoded = |i| {
        preceded::<_, _, _, (), _, _>(
            char('x'),
            map_res(take_while_m_n(2, 2, is_hex_digit), from_hex),
        )(i)
    };

    let escaped =
        |i| preceded(char('\\'), alt((encoded, char('\\'), char('"'))))(i);

    let token = |i| alt((escaped, satisfy(|c| c != '"')))(i);

    let string = |i| {
        map(delimited(char('"'), many0(token), char('"')), |v| {
            v.into_iter().collect::<String>()
        })(i)
    };

    string(s).unwrap().1
}

fn encode(s: &str) -> String {
    let mut ret = "\"".to_string();
    for c in s.chars() {
        if c == '"' || c == '\\' {
            ret.push('\\');
        }
        ret.push(c);
    }
    ret.push('"');
    ret
}

fn main() {
    let lines = stdin_lines().collect::<Vec<_>>();

    println!(
        "{}",
        lines
            .iter()
            .map(|line| line.chars().count() - parse(line).chars().count())
            .sum::<usize>()
    );

    println!(
        "{}",
        lines
            .iter()
            .map(|line| encode(line).chars().count() - line.chars().count())
            .sum::<usize>()
    );
}
