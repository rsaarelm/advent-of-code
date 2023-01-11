use aoc::prelude::*;

fn dist(a: IVec2, b: IVec2) -> i32 {
    (a - b).abs().dot(IVec2::ONE)
}

fn bounds_at(s: IVec2, b: IVec2, y: i32) -> Option<std::ops::Range<i32>> {
    let range = dist(s, b);
    let y_dist = (y - s.y).abs();
    if y_dist <= range {
        let d = range - y_dist;
        Some((s.x - d)..(s.x + d + 1))
    } else {
        None
    }
}

fn main() {
    let sensors: Vec<(IVec2, IVec2)> = stdin_lines()
        .map(|s| {
            let [sx, sy, bx, by]: [i32; 4] = fixed_numbers(s);
            (ivec2(sx, sy), ivec2(bx, by))
        })
        .collect();

    // Example uses different parameters than the actual input.
    let is_example = sensors[0] == (ivec2(2, 18), ivec2(-2, 15));

    // Part 1

    let scan_y = if is_example { 10 } else { 2_000_000 };

    let mut x_cover = HashSet::default();

    for &(s, b) in &sensors {
        for x in bounds_at(s, b, scan_y).unwrap_or(0..0) {
            if b != ivec2(x, scan_y) {
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
            if x >= 0 && x < boundary && !beacons.contains(&ivec2(x, y)) {
                println!("{}", x as i64 * 4_000_000 + y as i64);
                break 'scan;
            }
        }
    }
}
