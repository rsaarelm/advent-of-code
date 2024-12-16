use aoc::prelude::*;

fn main() {
    let (bounds, grid) = stdin_grid();
    let start = IVec2::from(bounds.get(idx_of(&grid, &'S').unwrap()));
    let end = IVec2::from(bounds.get(idx_of(&grid, &'E').unwrap()));

    let successors =
        |&(pos, dir): &(IVec2, IVec2)| -> Vec<((IVec2, IVec2), u32)> {
            let mut ret = Vec::new();
            if grid[bounds.idx(pos + dir)] != '#' {
                ret.push(((pos + dir, dir), 1));
            }
            ret.push(((pos, dir.cw()), 1000));
            ret.push(((pos, dir.ccw()), 1000));
            ret
        };

    let mut score = 0;
    let mut cover = HashSet::default();
    for node in dijkstra_search(successors, &(start, ivec2(1, 0))) {
        if node.item().0 != end {
            continue;
        }
        if score == 0 {
            // Start counting.
            score = node.total_cost();
        } else if score < node.total_cost() {
            // All optimal paths seen.
            break;
        }

        // Scan path cover.
        for ((p, _), _) in node.into_iter() {
            cover.insert(p);
        }
    }

    println!("{score}");
    println!("{}", cover.len());
}
