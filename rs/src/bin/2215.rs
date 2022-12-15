use std::collections::HashSet;

use aoc::prelude::*;

fn dist((a, b): (i64, i64), (c, d): (i64, i64)) -> i64 {
    (c - a).abs() + (d - b).abs()
}

fn bounds_at(s: (i64, i64), b: (i64, i64), y: i64) -> std::ops::Range<i64> {
    let range = dist(s, b);
    let y_dist = (y - s.1).abs();
    if y_dist <= range {
        let d = range - y_dist;
        (s.0 - d)..(s.0 + d + 1)
    } else {
        0..0
    }
}

fn main() {
    let sensors: Vec<((i64, i64), (i64, i64))> = stdin_lines()
        .map(|s| {
            let [sx, sy, bx, by]: [i64; 4] = fixed_numbers(s);
            ((sx, sy), (bx, by))
        })
        .collect();

    // Example uses different parameters than the actual input.
    let is_example = sensors[0] == ((2, 18), (-2, 15));

    // Part 1

    let scan_y = if is_example { 10 } else { 2_000_000 };

    let mut x_cover = HashSet::new();

    for &(s, b) in &sensors {
        for x in bounds_at(s, b, scan_y) {
            if b != (x, scan_y) {
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
            .filter_map(|&(s, b)| {
                let range = bounds_at(s, b, y);
                (!range.is_empty()).then_some(range)
            })
            .collect::<Vec<_>>();
        ranges.sort_by_key(|s| s.start);

        let mut end = ranges[0].end;

        for r in &ranges {
            if r.start <= end {
                end = end.max(r.end);
                continue;
            }
            let x = end;
            if x >= 0 && x < boundary && !beacons.contains(&(x, y)) {
                println!("{}", x * 4_000_000 + y);
                break 'scan;
            }
        }
    }
}

// Too low: 1602012312
