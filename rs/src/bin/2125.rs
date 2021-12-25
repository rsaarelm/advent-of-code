use aoc::prelude::*;

// Concepts
//
// Point on playing field
// field.right(point) -> point
// field.down(point) -> point
//
// because we need to know the varying widths for looping
// but points can also be flat now since pitch is also a thing
//
// field::horizontals() -> east facing
// field::verticals() -> south facing

fn main() {
    let (w, h, mut grid) = stdin_grid();

    for i in 0.. {
        let mut moves = 0;
        let mut grid2 = grid.clone();

        // Horizontals
        for y in 0..h {
            for x in 0..w {
                let x2 = (x + 1) % w;
                if grid[y][x] == '>' && grid[y][x2] == '.' {
                    moves += 1;
                    grid2[y][x] = '.';
                    grid2[y][x2] = '>';
                }
            }
        }

        grid = grid2;
        grid2 = grid.clone();

        // Verticals
        for y in 0..h {
            for x in 0..w {
                let y2 = (y + 1) % h;
                if grid[y][x] == 'v' && grid[y2][x] == '.' {
                    moves += 1;
                    grid2[y][x] = '.';
                    grid2[y2][x] = 'v';
                }
            }
        }

        grid = grid2;

        if moves == 0 {
            println!("{}", i + 1);
            break;
        }
    }
}
