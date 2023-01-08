use aoc::prelude::*;

fn number(mut i: &str) -> Result<(i64, &str), &str> {
    let sign;
    if i.starts_with('-') {
        sign = -1;
        i = &i[1..];
    } else {
        sign = 1;
    }

    let n = i
        .char_indices()
        .find_map(|(i, c)| (!c.is_numeric()).then_some(i))
        .unwrap_or(0);

    if n > 0 {
        Ok((sign * i[..n].parse::<i64>().unwrap(), &i[n..]))
    } else {
        Err(i)
    }
}

fn parse(i: &str) -> Result<((i64, i64), &str), &str> {
    if i.is_empty() {
        return Ok(((0, 0), ""));
    }

    let mut red_toggle = 1;

    let closing_char;
    if i.starts_with('[') {
        // array
        closing_char = ']';
    } else if i.starts_with('{') {
        closing_char = '}';
    } else {
        return Err(i);
    }

    let mut i = &i[1..];
    let mut n = 0;
    let mut with_red = 0;
    let mut non_red = 0;

    while !i.starts_with(closing_char) {
        if closing_char == '}' && i.starts_with("\"red\"") {
            red_toggle = 0;
            i = &i[5..];
            continue;
        }

        if let Ok(((a, b), rest)) = parse(i) {
            with_red += a;
            non_red += b;
            i = rest;
            continue;
        }

        if let Ok((x, rest)) = number(i) {
            n += x;
            i = rest;
            continue;
        }

        i = &i[1..];
    }

    Ok(((with_red + n, (non_red + n) * red_toggle), &i[1..]))
}

fn main() {
    let ((p1, p2), _) = parse(&stdin_string()).unwrap();
    println!("{p1}");
    println!("{p2}");
}
