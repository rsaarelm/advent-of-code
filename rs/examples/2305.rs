use std::ops::Range;

use itertools::Itertools;

use aoc::prelude::*;

fn remove_interval(
    set: &mut Vec<Range<i64>>,
    r: Range<i64>,
) -> Vec<Range<i64>> {
    // Invariant: Set is sorted and non-overlapping.

    // Cut-off elements.
    let mut parts = Vec::new();

    let mut i = 0;
    while i < set.len() && set[i].start < r.end {
        // No overlap, but we haven't reached the new range's area yet.
        if set[i].end <= r.start {
            i += 1;
        }
        // Full overlap, remove entire interval.
        else if r.start <= set[i].start && set[i].end <= r.end {
            parts.push(set[i].clone());
            set.remove(i);
            // Removed an element, so i stays the same.
        }
        // Cut in two
        else if set[i].start < r.start && r.end < set[i].end {
            let a = set[i].start..r.start;
            let b = r.end..set[i].end;
            set[i] = a;
            set.insert(i + 1, b);
            parts.push(r.clone());

            // r.end has been passed, exit
            break;
        }
        // Left half cut
        else if r.end < set[i].end {
            parts.push(set[i].start..r.end);
            set[i].start = r.end;

            // r.end has been passed, exit
            break;
        }
        // Right half cut, modify element and continue.
        else if r.start > set[i].start {
            parts.push(r.start..set[i].end);
            set[i].end = r.start;
            i += 1;
        }
        // Subsequent values are past the new range.
        else if set[i].start >= r.end {
            break;
        } else {
            unreachable!()
        }
    }

    parts
}

fn add_interval(set: &mut Vec<Range<i64>>, mut r: Range<i64>) {
    // Remove overlapping existing parts.
    let mut i = 0;
    while i < set.len() && set[i].start < r.end {
        // No overlap, but we haven't reached the new range's area yet.
        if set[i].end <= r.start {
            i += 1;
            continue;
        }

        // At least the left side of set[i] is now within r

        if set[i].start < r.start {
            r.start = set[i].start;
        }

        let a = set.remove(i);
        if a.end > r.end {
            r.end = a.end;
            break;
        }
    }

    set.push(r);
    set.sort_by_key(|a| a.start);
}

fn apply_map(set: &mut Vec<Range<i64>>, map: &[(Range<i64>, i64)]) {
    let mut inserts = Vec::new();
    for (r, offset) in map {
        for mut part in remove_interval(set, r.clone()) {
            part.start += offset;
            part.end += offset;
            inserts.push(part);
        }
    }

    for r in inserts {
        add_interval(set, r.clone());
    }
}

fn main() {
    let mut seeds: Vec<i64> = Vec::new();

    // (input range, delta)
    let mut maps: Vec<Vec<(Range<i64>, i64)>> = Vec::new();

    for line in stdin_lines() {
        if line.trim().is_empty() {
            continue;
        } else if line.contains("seeds") {
            seeds = numbers(line);
        } else if line.contains(':') {
            // Maps are always in same order, don't bother with labels.
            maps.push(Vec::new())
        } else {
            let n = maps.len() - 1;
            let [a, b, len] = fixed_numbers::<i64, 3>(line);
            maps[n].push((b..(b + len), (a - b)));
        }
    }

    let mut p1: Vec<Range<i64>> = seeds.iter().map(|&a| a..(a + 1)).collect();
    p1.sort_by_key(|a| a.start);
    let mut p2: Vec<Range<i64>> =
        seeds.iter().tuples().map(|(&a, &b)| a..(a + b)).collect();
    p2.sort_by_key(|a| a.start);

    for p in [p1, p2] {
        let mut set = p.clone();
        for map in &maps {
            apply_map(&mut set, map);
        }
        println!("{}", set[0].start);
    }
}
