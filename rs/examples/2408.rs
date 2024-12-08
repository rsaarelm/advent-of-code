use aoc::prelude::*;

fn main() {
    let mut bounds = Rect::default();
    let map: HashMap<IVec2, char> = stdin_grid_iter(&mut bounds)
        .filter_map(|(p, c)| (c != '.').then_some((p.into(), c)))
        .collect();

    let mut pairs = HashSet::default();
    // Assumption: Each pair will get inserted twice as (a, b) and (b, a).
    for (p, c) in &map {
        for q in map
            .iter()
            .filter_map(|(q, a)| (a == c && q != p).then_some(q))
        {
            pairs.insert((*p, *q));
        }
    }

    // P1

    let mut nodes = HashSet::default();
    for (p, q) in &pairs {
        let step = p - q;
        if bounds.contains(p + step) {
            nodes.insert(p + step);
        }
    }

    println!("{}", nodes.len());

    // P2

    let mut nodes = HashSet::default();
    for (p, q) in &pairs {
        let step = p - q;
        // Antinode appears at antenna now.
        let mut p = *p;
        while bounds.contains(p) {
            nodes.insert(p);
            p += step;
        }
    }

    println!("{}", nodes.len());
}
