use aoc::prelude::*;

fn main() {
    let input = stdin_string();

    let (a, o) = p1(&input);
    println!("{}", checksum(&a, &o));

    let a = p2(&input);
    println!("{}", checksum(&a, &o));

    // Cephalopods always sort their equations by number length, which has the
    // side effect of tricking humanoids into thinking about horizontal
    // ortography in left or right column alignment.

    // But the sort order isn't consistent. They alternate between
    // longest-first (left-alignment illusion) and shortest-first
    // (right-alignment illusion) seemingly at random. What's the pattern?
    // WHAT DO THEY KNOW WE DONT?

    /*
    #[derive(Copy, Clone, Debug)]
    enum Bias {
        Left,
        Right,
    }

    let mut code = Vec::new();
    for operands in &a {
        code.push(
            if operands[0].ilog10() > operands[operands.len() - 1].ilog10() {
                Some(Bias::Left)
            } else if operands[0].ilog10()
                < operands[operands.len() - 1].ilog10()
            {
                Some(Bias::Right)
            } else {
                // If all operands are the same length, we don't know it.
                None
            },
        );
    }

    eprint!("\x1b[1;32m");
    for (i, c) in code.iter().enumerate() {
        if i > 0 && i % 64 == 0 {
            eprintln!();
        } else if i > 0 && i % 8 == 0 {
            eprint!(" ");
        }
        match c {
            Some(Bias::Left) => eprint!("0"),
            Some(Bias::Right) => eprint!("1"),
            None => eprint!("-")
        }
    }
    eprintln!("\x1b[0m");
    */
}

fn p1(input: &str) -> (Vec<Vec<i64>>, Vec<String>) {
    let columns = numbers::<i64>(input.lines().next().unwrap()).len();
    let mut arguments: Vec<Vec<i64>> = vec![Vec::new(); columns];
    let mut operators: Vec<String> = Vec::new();
    for line in input.lines() {
        let num = numbers(&line);
        if !num.is_empty() {
            for (i, a) in num.iter().enumerate() {
                arguments[i].push(*a);
            }
        } else {
            operators = line.split_whitespace().map(|s| s.to_owned()).collect();
        }
    }
    (arguments, operators)
}

fn p2(input: &str) -> Vec<Vec<i64>> {
    let (bounds, buf) = grid(input);

    let mut arguments: Vec<Vec<i64>> = Vec::new();
    arguments.push(Vec::new());

    for x in 0..bounds.width() {
        let mut num = 0;
        for y in 0..(bounds.height() - 1) {
            let c = buf[bounds.idx([x, y])];
            if let Some(c) = c.to_digit(10) {
                num = num * 10 + c as i64;
            }
        }
        if num == 0 {
            // XXX: Assumption: Actual zero operands never shows up in input.
            arguments.push(Vec::new());
        } else {
            let i = arguments.len() - 1;
            arguments[i].push(num);
        }
    }
    if arguments[arguments.len() - 1].is_empty() {
        arguments.pop();
    }

    arguments
}

fn checksum(arguments: &Vec<Vec<i64>>, operators: &Vec<String>) -> i64 {
    let mut sum = 0;
    for (i, o) in operators.iter().enumerate() {
        match o.as_str() {
            "*" => sum += arguments[i].iter().product::<i64>(),
            "+" => sum += arguments[i].iter().sum::<i64>(),
            _ => panic!(),
        }
    }
    sum
}
