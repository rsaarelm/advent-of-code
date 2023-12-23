use aoc::prelude::*;

fn rim(
    bounds: &Rect<i32>,
    blocks: &HashSet<IVec2>,
    start: IVec2,
    n: usize,
) -> usize {
    dijkstra_map(
        |&p| {
            neighbors_4(p).filter(|&p| {
                !blocks.contains(&bounds.get(bounds.idx(p)).into())
            })
        },
        &start,
    )
    .skip_while(|(_, d)| *d < n)
    .take_while(|(_, d)| *d == n)
    .count()
}

fn area(
    bounds: &Rect<i32>,
    blocks: &HashSet<IVec2>,
    start: IVec2,
    n: usize,
) -> usize {
    ((n % 2)..=n)
        .step_by(2)
        .map(|n| rim(bounds, blocks, start, n))
        .sum::<usize>()
}

fn main() {
    let (bounds, buf) = grid(stdin_string());
    let mut blocks = HashSet::default();
    let mut start = ivec2(0, 0);
    for (i, c) in buf.iter().enumerate() {
        let pos = IVec2::from(bounds.get(i));
        match *c {
            '#' => {
                blocks.insert(pos);
            }
            'S' => {
                start = pos;
            }
            _ => {}
        }
    }

    let n = if bounds.width() == 11 {
        // Example
        6
    } else {
        64
    };

    println!("{}", area(&bounds, &blocks, start, n));

    // P2
    const STEPS: usize = 26501365;

    // Area repeats in a pattern for every map width multiple.
    let n = bounds.width() as usize;
    let k = STEPS % n;

    // Grab some data points. We need three points to derive a quadratic
    // polynomial.
    let mut xs = Vec::new();
    let mut ys = Vec::new();

    let mut area = 0;
    for i in (1..).step_by(2) {
        area += rim(&bounds, &blocks, start, i);
        if i % n == k {
            xs.push(i as f64);
            ys.push(area as f64);
        }
        if xs.len() == 3 {
            break;
        }
    }

    let [c, b, a] = &fit_polynomial(&xs, &ys)[..] else {
        panic!()
    };

    // And calculate ax^2 + bx + c for the target step count.
    let s = STEPS as f64;
    println!("{}", (a * s.powi(2) + b * s + c) as u64);
}
