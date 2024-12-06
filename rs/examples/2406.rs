use aoc::prelude::*;
use itertools::Itertools;

fn path(
    map: &HashSet<IVec2>,
    mut pos: IVec2,
) -> impl Iterator<Item = IVec2> + '_ {
    let mut dir = ivec2(0, -1);
    std::iter::from_fn(move || {
        // Turn away from obstacles (will loop forever if completely boxed in)
        while map.contains(&(pos + dir)) {
            dir = dir.cw();
        }

        pos += dir;
        Some(pos - dir)
    })
}

fn main() {
    let mut map: HashSet<IVec2> = Default::default();
    let mut pos: IVec2 = Default::default();
    let mut bounds: Rect<i32> = Default::default();

    for ([x, y], c) in stdin_grid_iter(&mut bounds) {
        match c {
            '#' => {
                map.insert(ivec2(x, y));
            }
            '^' => {
                pos = ivec2(x, y);
            }
            '.' => {}
            _ => panic!("Bad char {c}"),
        }
    }

    let default_path: HashSet<_> = path(&map, pos)
        .take_while(|&p| bounds.contains(p))
        .collect();

    println!("{}", default_path.len());

    let mut p2 = 0;
    for &p in &default_path {
        let mut map2 = map.clone();
        map2.insert(p);

        let mut seen: HashSet<(IVec2, IVec2)> = HashSet::default();
        // Get position + direction by tracking consecutive pairs of steps,
        // this lets us see when we've entered a loop.
        for step in path(&map2, pos)
            .take_while(|&p| bounds.contains(p))
            .tuple_windows()
        {
            if seen.contains(&step) {
                // Loop detected, mark this one down and continue scan.
                p2 += 1;
                break;
            }

            seen.insert(step);
        }
    }

    println!("{p2}");
}
