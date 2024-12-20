use std::collections::BTreeMap;

use aoc::prelude::*;

fn main() {
    let mut bounds = Default::default();
    let mut start = IVec2::default();
    let mut end = IVec2::default();
    let mut maze = HashSet::default();
    for (p, c) in stdin_grid_iter(&mut bounds) {
        let p = IVec2::from(p);
        match c {
            'S' => start = p,
            'E' => end = p,
            '#' => {
                maze.insert(p);
            }
            _ => {}
        }
    }

    let happy_path = grid_astar(&start, &end, |&p| {
        neighbors_4(p).filter(|p| !maze.contains(p))
    })
    .unwrap();

    let dist_to_end: HashMap<IVec2, usize> =
        bfs(|&p| neighbors_4(p).filter(|p| !maze.contains(p)), &end)
            .map(|(p, n)| (p, n + 1))
            .collect();

    for jump in [2, 20] {
        let mut cheats: BTreeMap<usize, usize> = BTreeMap::default();

        for i in 0..happy_path.len() {
            // Run up to i...
            let p = happy_path[i];

            // ...then teleport and see how close to finish line you got.
            for q in bounds.into_iter().map(IVec2::from).filter(|q| {
                *q != p && (*q - p).taxi_len() <= jump && !maze.contains(q)
            }) {
                let cheat_len =
                    i + (q - p).taxi_len() as usize + dist_to_end[&q];
                if cheat_len < happy_path.len() {
                    let saved = happy_path.len() - cheat_len;
                    *cheats.entry(saved).or_default() += 1;
                }
            }
        }

        println!(
            "{}",
            cheats
                .iter()
                .filter_map(|(t, n)| (*t >= 100).then_some(n))
                .sum::<usize>()
        );

        // Print summaries for example.
        // (The example gets zero points so we need to use tables to check
        // implementation).
        if bounds.width() == 15 {
            summary(&cheats);

            if jump == 2 {
                assert_eq!(
                    cheats,
                    BTreeMap::from([
                        (2, 14),
                        (4, 14),
                        (6, 2),
                        (8, 4),
                        (10, 2),
                        (12, 3),
                        (20, 1),
                        (36, 1),
                        (38, 1),
                        (40, 1),
                        (64, 1),
                    ])
                );
            }
        }
    }
}

fn summary(scores: &BTreeMap<usize, usize>) {
    eprintln!("--------------------------------");
    for (t, &n) in scores {
        if n > 1 {
            eprintln!("There are {n} cheats that save {t} picoseconds.");
        } else {
            eprintln!("There is one cheat that save {t} picoseconds.");
        }
    }
}
