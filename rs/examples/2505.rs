use std::ops::RangeInclusive;

use aoc::prelude::*;

fn main() {
    let mut items: Vec<i64> = Vec::new();
    let mut fresh: Vec<RangeInclusive<i64>> = Vec::new();
    for line in stdin_lines() {
        match &numbers(line)[..] {
            [] => continue,
            [a] => items.push(*a),
            [a, b] => fresh.push(*a..=*b),
            _ => panic!(),
        }
    }

    println!(
        "{}",
        items
            .iter()
            .filter(|i| fresh.iter().any(|f| f.contains(i)))
            .count()
    );

    let mut intervals = Vec::new();
    for i in &fresh {
        insert(&mut intervals, i)
    }

    println!(
        "{}",
        intervals
            .iter()
            .map(|a| a.end() - a.start() + 1)
            .sum::<i64>()
    );
}

/// Construct an ordered list of disjoint intervals.
fn insert(
    intervals: &mut Vec<RangeInclusive<i64>>,
    item: &RangeInclusive<i64>,
) {
    for i in 0..intervals.len() {
        if item.end() < intervals[i].start() {
            intervals.insert(i, item.clone());
            return;
        }
        if let Some(merged) = merge(&intervals[i], &item) {
            intervals.remove(i);
            return insert(intervals, &merged);
        }
        assert!(item.start() > intervals[i].start());
    }

    assert!(
        intervals.is_empty()
            || intervals[intervals.len() - 1].end() < item.start()
    );
    intervals.push(item.clone());
}

fn merge(
    a: &RangeInclusive<i64>,
    b: &RangeInclusive<i64>,
) -> Option<RangeInclusive<i64>> {
    // Ensure a.start() <= b.start().
    let (a, b) = if b.start() < a.start() {
        (b, a)
    } else {
        (a, b)
    };

    if b.start() <= a.end() {
        Some(*a.start().min(b.start())..=*a.end().max(b.end()))
    } else {
        None
    }
}
