use aoc::prelude::*;
use pathfinding::prelude::yen;

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
    for (path, c) in yen(
        &(start, ivec2(1, 0)),
        successors,
        |&(pos, _)| pos == end,
        // Guess the upper bound for number of optimal paths:
        9,
    ) {
        if score == 0 {
            // Start counting.
            score = c;
        } else if score < c {
            // All optimal paths seen.
            break;
        }

        // Scan path cover.
        for (p, _) in path {
            cover.insert(p);
        }
    }

    println!("{score}");
    println!("{}", cover.len());
}
