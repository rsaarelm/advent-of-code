use std::collections::HashSet;

use aoc::prelude::*;

fn bounds_at(s: Vec2, b: Vec2, y: i64) -> Option<std::ops::Range<i64>> {
    let range = (s - b).taxicab_len();
    let y_dist = (y - s.y).abs();
    if y_dist <= range {
        let d = range - y_dist;
        Some((s.x - d)..(s.x + d + 1))
    } else {
        None
    }
}

fn main() {
    let sensors: Vec<(Vec2, Vec2)> = stdin_lines()
        .map(|s| {
            let [sx, sy, bx, by]: [i64; 4] = fixed_numbers(s);
            (vec2(sx, sy), vec2(bx, by))
        })
        .collect();

    // Example uses different parameters than the actual input.
    let is_example = sensors[0] == (vec2(2, 18), vec2(-2, 15));

    // Part 1

    let scan_y = if is_example { 10 } else { 2_000_000 };

    let mut x_cover = HashSet::new();

    for &(s, b) in &sensors {
        for x in bounds_at(s, b, scan_y).unwrap_or(0..0) {
            if b != vec2(x, scan_y) {
                x_cover.insert(x);
            }
        }
    }

    println!("{}", x_cover.len());

    // Part 2

    let boundary = if is_example { 20 } else { 4_000_000 };

    let beacons = sensors.iter().map(|&(_, b)| b).collect::<HashSet<_>>();

    'scan: for y in 0..=boundary {
        let mut ranges = sensors
            .iter()
            .filter_map(|&(s, b)| bounds_at(s, b, y))
            .collect::<Vec<_>>();
        ranges.sort_by_key(|s| s.start);

        let mut end = ranges[0].end;

        for r in &ranges {
            if r.start <= end {
                end = end.max(r.end);
                continue;
            }
            let x = end;
            if x >= 0 && x < boundary && !beacons.contains(&vec2(x, y)) {
                println!("{}", x * 4_000_000 + y);
                break 'scan;
            }
        }
    }
}
