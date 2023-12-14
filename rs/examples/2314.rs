use aoc::prelude::*;

fn step(bounds: &Rect<i32>, buf: &mut [char], dir: IVec2) -> usize {
    let mut changes = 0;
    for p in *bounds {
        let p = IVec2::from(p);
        if !bounds.contains(p + dir) {
            continue;
        }

        let a = bounds.idx(p);
        let b = bounds.idx(p + dir);
        if buf[a] == 'O' && buf[b] == '.' {
            buf.swap(a, b);
            changes += 1;
        }
    }

    changes
}

fn tilt(bounds: &Rect<i32>, buf: &mut [char], dir: IVec2) {
    while step(bounds, buf, dir) != 0 {}
}

fn weight(bounds: &Rect<i32>, buf: &[char]) -> i32 {
    let mut ret = 0;
    for p in *bounds {
        let p = IVec2::from(p);
        let c = bounds.height() - p.y;
        if buf[bounds.idx(p)] == 'O' {
            ret += c;
        }
    }

    ret
}

fn main() {
    let (bounds, buf) = grid(stdin_string());

    // P1

    let mut p1 = buf.clone();
    tilt(&bounds, &mut p1, DIR_4[UP]);
    println!("{}", weight(&bounds, &p1));

    // P2

    const N: usize = 1_000_000_000;
    let mut p2 = buf.clone();

    let mut sequence: IndexMap<Vec<char>, (usize, i32)> = Default::default();
    sequence.insert(buf.clone(), (0, weight(&bounds, &buf)));

    for i in 1..N {
        for d in [UP, LEFT, DOWN, RIGHT] {
            tilt(&bounds, &mut p2, DIR_4[d]);
        }

        // Look for a repeat pattern so we can loop the values instead of
        // running to 1 000 000 000.
        if let Some((j, _)) = sequence.get(&p2) {
            eprintln!("cycle found: {i} -> {j}");
            let reps = N - j;
            let idx = j + (reps % (i - j));
            println!("{}", sequence.get_index(idx).unwrap().1 .1);
            break;
        }
        sequence.insert(p2.clone(), (i, weight(&bounds, &p2)));
    }
}
