use aoc::prelude::*;

fn number(mut input: &[u8]) -> Option<(usize, &[u8])> {
    if input.is_empty() || !(input[0] as char).is_digit(10) {
        return None;
    }

    let mut ret = 0;

    loop {
        let c = input[0] as char;
        if let Some(digit) = c.to_digit(10) {
            ret = ret * 10 + digit as usize;
            input = &input[1..];
        } else {
            return Some((ret, input));
        }

        if input.is_empty() {
            return Some((ret, input));
        }
    }
}

fn tok(input: &[u8], tok: char) -> Option<&[u8]> {
    if !input.is_empty() && input[0] as char == tok {
        Some(&input[1..])
    } else {
        None
    }
}

fn marker(input: &[u8]) -> Option<(usize, usize, &[u8])> {
    let input = tok(input, '(')?;
    let (a, input) = number(input)?;
    let input = tok(input, 'x')?;
    let (b, input) = number(input)?;
    let input = tok(input, ')')?;
    Some((a, b, input))
}

fn calc(mut i: &[u8], recurse: bool) -> usize {
    let mut n = 0;
    while !i.is_empty() {
        if let Some((len, x, rest)) = marker(i) {
            i = &rest[len..];
            let len = if recurse {
                calc(&rest[..len], recurse)
            } else {
                len
            };
            n += len * x;
        } else {
            i = &i[1..];
            n += 1;
        }
    }
    n
}

fn main() {
    let input = stdin_string();

    println!("{}", calc(input.as_bytes(), false));
    println!("{}", calc(input.as_bytes(), true));
}
