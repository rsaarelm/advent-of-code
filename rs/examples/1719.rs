use aoc::prelude::*;

fn cursor(bounds: Rect<i32>, buf: &[char]) -> impl Iterator<Item = char> + '_ {
    let mut dir = DOWN;
    let mut pos = ivec2(
        buf.iter().position(|c| !c.is_whitespace()).unwrap() as i32,
        0,
    );
    assert!(bounds.contains(pos));

    std::iter::from_fn(move || {
        if !bounds.contains(pos) || buf[bounds.idx(pos)].is_whitespace() {
            return None;
        }

        let c = buf[bounds.idx(pos)];
        if c == '+' {
            // Look for direction to turn for line continuation.
            if !buf[bounds.idx(pos + DIR_4[(dir + 1) % 4])].is_whitespace() {
                dir = (dir + 1) % 4;
            } else if !buf[bounds.idx(pos + DIR_4[(dir + 3) % 4])]
                .is_whitespace()
            {
                dir = (dir + 3) % 4;
            }
        }

        pos += DIR_4[dir];

        Some(c)
    })
}

fn main() {
    let (bounds, buf) = stdin_grid();

    println!(
        "{}",
        cursor(bounds, &buf)
            .filter(|c| c.is_alphabetic())
            .collect::<String>()
    );

    println!(
        "{}",
        cursor(bounds, &buf)
            .enumerate()
            .filter(|(_, c)| c.is_alphabetic())
            .last()
            .unwrap()
            .0
            + 1
    );
}
