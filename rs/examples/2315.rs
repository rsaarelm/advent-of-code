use aoc::prelude::*;

fn hash(s: &str) -> usize {
    let mut ret = 0;
    for c in s.chars() {
        ret += c as usize;
        ret *= 17;
        ret %= 0x100;
    }

    ret
}

fn main() {
    assert_eq!(hash("HASH"), 52);
    let input: Vec<String> =
        stdin_string().split(',').map(|a| a.to_owned()).collect();

    // P1
    println!("{}", input.iter().map(|a| hash(a)).sum::<usize>());

    // P2

    let mut store: [Vec<(String, usize)>; 256] =
        std::array::from_fn(|_| Vec::new());

    for elt in &input {
        if let Ok(a) = re_parser::<String>("(.*)-")(elt) {
            let idx = hash(&a);
            if let Some(pos) =
                store[idx].iter().position(|(x, _)| x == a.as_str())
            {
                store[idx].remove(pos);
            }
        } else if let Ok((a, n)) =
            re_parser::<(String, usize)>("(.*)=(.*)")(elt)
        {
            let idx = hash(&a);
            if let Some(pos) =
                store[idx].iter().position(|(x, _)| x == a.as_str())
            {
                store[idx][pos] = (a, n);
            } else {
                store[idx].push((a, n));
            }
        } else {
            panic!()
        }
    }

    let mut n = 0;
    for (i, a) in store.iter().enumerate() {
        for (j, (_, x)) in a.iter().enumerate() {
            n += (i + 1) * (j + 1) * x;
        }
    }

    println!("{n}");
}
