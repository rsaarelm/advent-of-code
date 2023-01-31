use aoc::prelude::*;

fn main() {
    let (bounds, mut grid) = stdin_grid();

    for i in 0.. {
        let mut moves = 0;
        let mut grid2 = grid.clone();

        // Horizontals
        for p in bounds {
            let p = IVec2::from(p);
            if grid[bounds.idx(p)] == '>'
                && grid[bounds.idx(p + ivec2(1, 0))] == '.'
            {
                moves += 1;
                grid2[bounds.idx(p)] = '.';
                grid2[bounds.idx(p + ivec2(1, 0))] = '>';
            }
        }

        grid = grid2;
        grid2 = grid.clone();

        // Verticals
        for p in bounds {
            let p = IVec2::from(p);
            if grid[bounds.idx(p)] == 'v'
                && grid[bounds.idx(p + ivec2(0, 1))] == '.'
            {
                moves += 1;
                grid2[bounds.idx(p)] = '.';
                grid2[bounds.idx(p + ivec2(0, 1))] = 'v';
            }
        }

        grid = grid2;

        if moves == 0 {
            println!("{}", i + 1);
            break;
        }
    }
}
