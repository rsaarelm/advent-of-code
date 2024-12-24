use itertools::Itertools;

use aoc::prelude::*;

fn main() {
    let input = stdin_string();

    // List host names.
    let mut hosts = HashSet::default();
    for line in input.lines() {
        let (a, b) = line.split_once('-').unwrap();
        hosts.insert(a.to_owned());
        hosts.insert(b.to_owned());
    }

    let mut hosts = hosts.into_iter().collect::<Vec<String>>();
    hosts.sort();

    // Build graph.
    let mut net = vec![false; hosts.len() * hosts.len()];
    let bounds = area(hosts.len(), hosts.len());

    for line in input.lines() {
        let (a, b) = line.split_once('-').unwrap();
        let a = hosts.iter().position(|x| x == a).unwrap();
        let b = hosts.iter().position(|x| x == b).unwrap();

        net[bounds.idx([a, b])] = true;
        net[bounds.idx([b, a])] = true;
    }

    // Is-connected predicate
    let n = |a, b| net[bounds.idx([a, b])];

    // P1
    let mut triples = HashSet::default();
    for (i, _) in hosts.iter().enumerate().filter(|(_, t)| t.starts_with('t')) {
        for mut ps in (0..hosts.len()).filter(|&j| n(i, j)).permutations(2) {
            if !n(ps[0], ps[1]) {
                continue;
            }
            ps.push(i);
            ps.sort();
            triples.insert(ps);
        }
    }

    println!("{}", triples.len());

    // P2
    let mut comp = Vec::new();

    for i in 0..hosts.len() {
        let mut c = vec![i];
        for j in (0..hosts.len()).filter(|&j| n(i, j)) {
            if c.iter().all(|&i| n(i, j)) {
                c.push(j);
            }
        }

        if c.len() > comp.len() {
            comp = c;
        }
    }

    println!("{}", comp.iter().map(|&i| &hosts[i]).join(","));
}
