use aoc::prelude::*;

fn main() {
    let mut input: Vec<String> = Vec::new();

    || -> Option<()> {
        let mut lines = stdin_lines();
        loop {
            input.push(lines.by_ref().take(7).collect::<Vec<_>>().join(""));
            lines.next()?;
        }
    }();

    let mut p1 = 0;

    for i in 0..input.len() {
        for j in (i + 1)..input.len() {
            if input[i]
                .chars()
                .zip(input[j].chars())
                .all(|(a, b)| a == '.' || b == '.')
            {
                p1 += 1;
            }
        }
    }

    println!("{p1}");
}
