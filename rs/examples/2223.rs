use aoc::prelude::*;

fn main() {
    let mut elves = HashSet::default();
    for (y, line) in stdin_lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if !c.is_whitespace() && c != '.' {
                elves.insert(ivec2(x as i32, y as i32));
            }
        }
    }

    let mut dirs = vec![UP, DOWN, LEFT, RIGHT];
    for round in 1.. {
        let mut come_from: HashMap<IVec2, Vec<IVec2>> = Default::default();
        let prev_elves = elves.clone();

        'elfscan: for &elf in &elves {
            if !DIR_8.iter().any(|&d| elves.contains(&(elf + d))) {
                // No neighbors, stay put.
                come_from.entry(elf).or_default().push(elf);
                continue;
            }

            'scan: for &d in &dirs {
                for v in [
                    DIR_8[d * 2],
                    DIR_8[(d * 2 + 7) % 8],
                    DIR_8[(d * 2 + 1) % 8],
                ] {
                    if elves.contains(&(elf + v)) {
                        continue 'scan;
                    }
                }
                come_from.entry(elf + DIR_4[d]).or_default().push(elf);
                continue 'elfscan;
            }
            come_from.entry(elf).or_default().push(elf);
        }

        for (p, movers) in come_from {
            if let [elf] = movers[..] {
                elves.remove(&elf);
                elves.insert(p);
            }
        }

        let d = dirs.remove(0);
        dirs.push(d);

        // Part 1
        if round == 11 {
            let zone = Rect::from_points_inclusive(elves.iter().copied());
            println!("{}", zone.volume() as usize - elves.len());
        }

        if elves == prev_elves {
            println!("{round}");
            break;
        }
    }
}
