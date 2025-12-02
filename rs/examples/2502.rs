use aoc::prelude::*;

fn is_bad(num: i64) -> bool {
    let num = num.to_string();
    if num.len() % 2 == 1 {
        return false;
    }
    let (fst, snd) = num.split_at(num.len() / 2);
    fst == snd
}

fn is_bad2(num: i64) -> bool {
    let num = num.to_string();
    for i in 1..=num.len()/2 {
        if num.len() % i == 0 {
            let (prefix, _) = num.split_at(i);
            if num == prefix.repeat(num.len() / i) {
                return true;
            }
        }
    }
    false
}

fn main() {
    // XXX: Need to mangle the string so hyphens aren't parsed as minuses.
    let input: Vec<i64> = numbers(stdin_string().replace("-", " "));

    let mut ret = 0;
    for r in input.chunks(2).map(|c| c[0]..=c[1]) {
        for n in r.filter(|&x| is_bad(x)) {
            ret += n;
        }
    }
    println!("{ret}");

    let mut ret = 0;
    for r in input.chunks(2).map(|c| c[0]..=c[1]) {
        for n in r.filter(|&x| is_bad2(x)) {
            ret += n;
        }
    }
    println!("{ret}");
}

