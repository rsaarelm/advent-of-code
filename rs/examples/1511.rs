use aoc::prelude::*;

fn inc(password: &mut [u8]) {
    for c in password.iter_mut().rev() {
        if *c == b'z' {
            *c = b'a';
        } else {
            *c += 1;
            break;
        }
    }
}

fn is_valid(password: &[u8]) -> bool {
    let mut triple_found = false;
    let mut dub_count = 0;
    // Used to ensure dubs don't overlap.
    let mut last_dub_pos = 0;

    for (i, c) in password.iter().enumerate() {
        if matches!(*c, b'i' | b'o' | b'l') {
            return false;
        }

        if i > 0 && last_dub_pos + 1 != i && password[i - 1] == *c {
            last_dub_pos = i;
            dub_count += 1;
        }

        if password.len() >= i + 3
            && password[i + 1] == c + 1
            && password[i + 2] == c + 2
        {
            triple_found = true;
        }
    }

    triple_found && dub_count >= 2
}

fn main() {
    for pw in (0..)
        .scan(stdin_string().into_bytes(), |pw, _| {
            inc(&mut *pw);
            if is_valid(pw) {
                Some(Some(pw.clone()))
            } else {
                Some(None)
            }
        })
        .flatten()
        .take(2)
    {
        println!("{}", String::from_utf8(pw).unwrap());
    }
}
