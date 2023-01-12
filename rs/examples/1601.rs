use aoc::prelude::*;

fn main() {
    let path = {
        let mut d = 0;
        let mut p = ivec2(0, 0);
        let mut path = vec![p];
        for step in stdin_string().split(", ") {
            d += if step.starts_with('L') { 3 } else { 1 };
            d %= 4;
            for _ in 0..(step[1..].parse::<usize>().unwrap()) {
                p += DIR_4[d];
                path.push(p);
            }
        }
        path
    };

    // Part 1
    println!("{}", path[path.len() - 1].taxi_len());

    // Part 2
    let mut seen = HashSet::default();
    for &p in &path {
        if seen.contains(&p) {
            println!("{}", p.taxi_len());
            break;
        }
        seen.insert(p);
    }
}
