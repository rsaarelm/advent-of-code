use aoc::prelude::*;

fn main() {
    let mut input: Vec<Vec<(u32, u32, u32)>> = Vec::new();

    for line in stdin_lines() {
        let line = line.split(": ").nth(1).unwrap();
        let (mut r, mut g, mut b) = (0, 0, 0);
        let mut play = Vec::new();
        for draw in line.split("; ") {
            for elt in draw.split(", ") {
                let n = numbers(elt)[0];
                if elt.contains("red") {
                    r = n;
                } else if elt.contains("green") {
                    g = n;
                } else if elt.contains("blue") {
                    b = n;
                } else {
                    panic!("Bad element");
                }
            }
            play.push((r, g, b));
        }
        input.push(play);
    }

    let mut p1 = 0;
    let mut p2 = 0;

    for (i, play) in input.iter().enumerate() {
        let (r, g, b) = play
            .iter()
            .copied()
            .reduce(|(a, b, c), (i, j, k)| (a.max(i), b.max(j), c.max(k)))
            .unwrap();

        if r <= 12 && g <= 13 && b <= 14 {
            p1 += i + 1;
        }

        p2 += r * g * b;
    }

    println!("{p1}");
    println!("{p2}");
}
