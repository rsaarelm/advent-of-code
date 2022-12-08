use aoc::prelude::*;

fn sweep(grid: &Vec<Vec<char>>, view: &mut Vec<Vec<char>>,
    width: usize, height: usize,
    x0: usize, y0: usize, dx: i32, dy: i32) {
    view[y0][x0] = '1';
    let mut cover = grid[y0][x0].to_digit(10).unwrap();

    let (mut x, mut y) = (x0 as i32 + dx, y0 as i32 + dy);
    while x >= 0 && y >= 0 && x < width as i32 && y < height as i32 {
        let new_cover = grid[y as usize][x as usize].to_digit(10).unwrap();

        if new_cover > cover {
            view[y as usize][x as usize] = '1';
            cover = new_cover;
        }

        x += dx;
        y += dy;
    }

}

fn main() {
    let (width, height, grid) = stdin_grid();

    // TODO: map grid to integers, stdin_grid_mapped

    let mut view = grid.clone();
    for y in 0..height {
        for x in 0..width {
            view[y][x] = '.';
        }
    }

    for x in 0..width {
        sweep(&grid, &mut view, width, height, x, 0, 0, 1);
        sweep(&grid, &mut view, width, height, x, height - 1, 0, -1);
    }

    for y in 0..height {
        sweep(&grid, &mut view, width, height, 0, y, 1, 0);
        sweep(&grid, &mut view, width, height, width - 1, y, -1, 0);
    }

    let mut seen = 0;
    for y in 0..height {
        for x in 0..width {
            if view[y][x] != '.' {
                seen += 1;
            }
        }
    }

    println!("{}", seen);
}
