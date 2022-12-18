use aoc::prelude::*;

/// Iterate cells along a line on the grid.
fn sweep(
    grid: &'_ Vec<Vec<char>>,
    x0: usize,
    y0: usize,
    dx: i32,
    dy: i32,
) -> impl Iterator<Item = ((usize, usize), i32)> + '_ {
    let height = grid.len() as i32;
    let width = grid[0].len() as i32;

    let mut x = x0 as i32;
    let mut y = y0 as i32;
    std::iter::from_fn(move || {
        if x < 0 || y < 0 || x >= width || y >= height {
            None
        } else {
            let ret = (
                (x as usize, y as usize),
                grid[y as usize][x as usize].to_digit(10).unwrap() as i32,
            );

            x += dx;
            y += dy;
            Some(ret)
        }
    })
}

// Line of sight updating closure generator.
fn line_of_sight(
    view: &'_ mut Vec<Vec<usize>>,
) -> impl FnMut(((usize, usize), i32)) + '_ {
    let mut current = -1;
    move |((x, y), h)| {
        if h > current {
            view[y][x] = 1;
            current = h;
        }
    }
}

fn score_run(run: impl Iterator<Item = ((usize, usize), i32)>) -> usize {
    let mut h = -1;
    let mut ret = 0;
    for ((_, _), a) in run {
        if h < 0 {
            // Height of starting tree.
            h = a;
            continue;
        }
        ret += 1;
        if a >= h {
            // Hit another tree as tall as starting tree, stop.
            break;
        }
    }
    ret
}

fn main() {
    let (width, height, grid) = stdin_grid();

    let mut view = vec![vec![0; width]; height];

    // Do line of sight sweeps that update the view from the edges.
    for x in 0..width {
        sweep(&grid, x, 0, 0, 1).for_each(line_of_sight(&mut view));
        sweep(&grid, x, height - 1, 0, -1).for_each(line_of_sight(&mut view));
    }

    for y in 0..height {
        sweep(&grid, 0, y, 1, 0).for_each(line_of_sight(&mut view));
        sweep(&grid, width - 1, y, -1, 0).for_each(line_of_sight(&mut view));
    }

    let mut seen = 0;
    for y in 0..height {
        for x in 0..width {
            if view[y][x] != 0 {
                seen += 1;
            }
        }
    }

    println!("{}", seen);

    let mut best: usize = 0;
    // Score sweeps towards the edges from each point.
    for y in 0..height {
        for x in 0..width {
            let score = score_run(sweep(&grid, x, y, -1, 0))
                * score_run(sweep(&grid, x, y, 1, 0))
                * score_run(sweep(&grid, x, y, 0, -1))
                * score_run(sweep(&grid, x, y, 0, 1));
            best = score.max(best);
        }
    }

    println!("{}", best);
}
