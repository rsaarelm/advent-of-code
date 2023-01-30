use aoc::prelude::*;

/// Iterate cells along a line on the grid.
fn sweep<'a>(
    bounds: &'a NRange<i32, 2>,
    grid: &'a Vec<char>,
    mut p: IVec2,
    d: IVec2,
) -> impl Iterator<Item = (IVec2, i32)> + 'a {
    std::iter::from_fn(move || {
        if !bounds.contains(p) {
            None
        } else {
            let ret = (p, grid[bounds.idx(p)].to_digit(10).unwrap() as i32);

            p += d;
            Some(ret)
        }
    })
}

// Line of sight updating closure generator.
fn line_of_sight<'a>(
    bounds: &'a NRange<i32, 2>,
    view: &'a mut Vec<usize>,
) -> impl FnMut((IVec2, i32)) + 'a {
    let mut current = -1;
    move |(p, h)| {
        if h > current {
            view[bounds.idx(p)] = 1;
            current = h;
        }
    }
}

fn score_run(run: impl Iterator<Item = (IVec2, i32)>) -> usize {
    let mut h = -1;
    let mut ret = 0;
    for (_, a) in run {
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
    let (bounds, grid) = stdin_grid();

    let mut view = vec![0; grid.len()];

    // Do line of sight sweeps that update the view from the edges.
    for x in 0..bounds.width() {
        sweep(&bounds, &grid, ivec2(x, 0), ivec2(0, 1))
            .for_each(line_of_sight(&bounds, &mut view));
        sweep(&bounds, &grid, ivec2(x, bounds.height() - 1), ivec2(0, -1))
            .for_each(line_of_sight(&bounds, &mut view));
    }

    for y in 0..bounds.height() {
        sweep(&bounds, &grid, ivec2(0, y), ivec2(1, 0))
            .for_each(line_of_sight(&bounds, &mut view));
        sweep(&bounds, &grid, ivec2(bounds.width() - 1, y), ivec2(-1, 0))
            .for_each(line_of_sight(&bounds, &mut view));
    }

    let seen = bounds
        .into_iter()
        .filter(|&p| view[bounds.idx(p)] != 0)
        .count();
    println!("{}", seen);

    let mut best: usize = 0;
    // Score sweeps towards the edges from each point.
    for p in bounds {
        let score = DIR_4
            .iter()
            .map(|&d| score_run(sweep(&bounds, &grid, p.into(), d)))
            .product::<usize>();
        best = score.max(best);
    }

    println!("{}", best);
}
