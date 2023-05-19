use aoc::prelude::*;

type Grid = (Rect<i32>, Vec<char>);

fn clip(grid: &Grid, window: Rect<i32>) -> Grid {
    let mut buf = vec!['\0'; window.volume() as usize];
    for p in window {
        buf[window.idx(p)] = grid.1[grid.0.idx(p)];
    }
    (window, buf)
}

fn enhance(
    codebook: &HashMap<Vec<char>, Vec<char>>,
    grid @ (bounds, buf): &Grid,
) -> Grid {
    let step = if bounds.width() % 2 == 0 { 2 } else { 3 };

    let mut buf_2 =
        vec!['\0'; buf.len() / (step * step) * ((step + 1) * (step + 1))];

    let window_1 = area(step as i32, step as i32);
    let window_2 = area(step as i32 + 1, step as i32 + 1);

    let bounds_2 = area(
        bounds.width() / step as i32 * (step + 1) as i32,
        bounds.height() / step as i32 * (step + 1) as i32,
    );

    for y in (0..bounds.height()).step_by(step) {
        for x in (0..bounds.width()).step_by(step) {
            let step = step as i32;

            let p1 = [x, y];
            let window_1 = window_1 + p1;

            let p2 = [(x / step * (step + 1)), (y / step * (step + 1))];
            let window_2 = window_2 + p2;

            let pat = clip(grid, window_1);

            let output = &codebook[&pat.1];
            for p in window_2 {
                buf_2[bounds_2.idx(p)] = output[window_2.idx(p)];
            }
        }
    }

    (bounds_2, buf_2)
}

fn main() {
    let mut codebook = HashMap::default();
    for line in stdin_lines() {
        let [a, b]: [&str; 2] =
            line.split(" => ").collect::<Vec<_>>().try_into().unwrap();
        let mut a = grid(a.replace('/', "\n"));
        let b = grid(b.replace('/', "\n"));

        // Insert the same output result in the codebook for the four
        // rotations of the two mirrorings of the input grid.
        for _ in 0..2 {
            let mut a2 = a.clone();
            for _ in 0..4 {
                codebook.insert(a2.1.clone(), b.1.clone());
                a2 = rotate_grid(a2);
            }
            a = mirror_grid(a);
        }
    }
    let seed = grid(
        "\
.#.
..#
###",
    );

    let mut grid = seed.clone();
    for i in 1..=18 {
        grid = enhance(&codebook, &grid);
        if i == 5 {
            println!("{}", grid.1.iter().filter(|&&c| c == '#').count());
        }
    }
    println!("{}", grid.1.iter().filter(|&&c| c == '#').count());
}
