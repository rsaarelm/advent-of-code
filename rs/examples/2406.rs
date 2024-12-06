use aoc::prelude::*;

// Return length for exit, None if path loops.
fn path(bounds: &Rect<i32>, map: &HashSet<IVec2>, mut pos: IVec2) -> Option<HashMap<IVec2, Vec<IVec2>>> {
    let mut dir = ivec2(0, -1);
    // Footprint, tracking directions it was traversed in.
    let mut posns: HashMap<IVec2, Vec<IVec2>> = HashMap::default();

    loop {
        let prevs = posns.entry(pos).or_default();
        if prevs.contains(&dir) {
            // Loop detected, bail.
            return None;
        }
        prevs.push(dir);

        // Turn away from obstacles (will loop forever if completely boxed in)
        while map.contains(&(pos + dir)) {
            dir = dir.cw();
        }
        if !bounds.contains(pos + dir) {
            return Some(posns);
        }
        pos += dir;
    }
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

    let default_path = path(&bounds, &map, pos).unwrap();

    println!("{}", default_path.len());

    let mut p2 = 0;
    for &p in default_path.keys() {
        let mut map2 = map.clone();
        map2.insert(p);
        if path(&bounds, &map2, pos).is_none() {
            p2 += 1;
        }
    }

    println!("{p2}");
}
