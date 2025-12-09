use std::collections::BTreeSet;

use aoc::prelude::*;
use itertools::Itertools;

fn main() {
    let input: Vec<I64Vec2> =
        stdin_lines().map(|a| fixed_numbers(a).into()).collect();

    // Make a compressed map of the path, we only care about when we're
    // inside, not about the exact distances. The distances make naive
    // computation ineffective.
    let (x_map, y_map) = {
        let xs: BTreeSet<i64> = input.iter().map(|p| p.x).collect();
        let ys: BTreeSet<i64> = input.iter().map(|p| p.y).collect();

        // Check that there are no lines hugging each other and we're correct
        // to put a 1 unit gap between everything.
        assert!(xs
            .iter()
            .cloned()
            .collect::<Vec<_>>()
            .into_iter()
            .tuple_windows()
            .all(|(a, b)| b - a > 1));
        assert!(ys
            .iter()
            .cloned()
            .collect::<Vec<_>>()
            .into_iter()
            .tuple_windows()
            .all(|(a, b)| b - a > 1));

        let mut x_map = HashMap::default();
        let mut y_map = HashMap::default();

        for (i, &x) in xs.iter().enumerate() {
            x_map.insert(x, i as i64 * 2);
        }

        for (i, &y) in ys.iter().enumerate() {
            y_map.insert(y, i as i64 * 2);
        }

        (x_map, y_map)
    };

    // Shape drawn by the input nodes in compressed space.
    let mut shape = HashSet::default();

    // Trace the boundaries of the compressed map.
    for (a, b) in input.iter().circular_tuple_windows() {
        let mut a = i64vec2(x_map[&a.x], y_map[&a.y]);
        let b = i64vec2(x_map[&b.x], y_map[&b.y]);

        assert!(a.x == b.x || a.y == b.y);

        // Trace the span from a to b in the shape.
        let inc = (b - a).signum();
        loop {
            shape.insert(a);
            if a == b {
                break;
            }
            a += inc;
        }
    }

    // Probe for a fill-in point, we know the minimum coordinate is 0 and the
    // next is 2, so probing at y=1 should find us a wall to breach.
    let mut fill_start = I64Vec2::default();
    for x in 0.. {
        let p = i64vec2(x, 1);
        if shape.contains(&p) {
            fill_start = i64vec2(x + 1, 1);
            assert!(!shape.contains(&fill_start));
            break;
        }
    }

    // Flood fill the insides.
    for p in bfs(
        |&p| neighbors_4(p).filter(|x| !shape.contains(x)),
        &fill_start,
    )
    .map(|(p, _)| p)
    .collect::<Vec<_>>()
    {
        shape.insert(p);
    }

    let mut p1 = 0;
    let mut p2 = 0;
    for (i, a) in input.iter().enumerate() {
        'scan: for b in input.iter().skip(i + 1) {
            let span = Rect::from_points_inclusive([*a, *b]);
            p1 = p1.max(span.volume());

            let a = i64vec2(x_map[&a.x], y_map[&a.y]);
            let b = i64vec2(x_map[&b.x], y_map[&b.y]);
            let compressed_span = Rect::from_points_inclusive([a, b]);
            for p in compressed_span {
                if !shape.contains(&p.into()) {
                    continue 'scan;
                }
            }

            p2 = p2.max(span.volume());
        }
    }
    println!("{p1}");
    println!("{p2}");
}
