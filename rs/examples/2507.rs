use aoc::prelude::*;
use memoize::memoize;

fn main() {
    let mut origin: IVec2 = Default::default();
    let mut splitters: Vec<IVec2> = Default::default();
    let mut bounds = Rect::default();

    for (pos, c) in stdin_grid_iter(&mut bounds) {
        match c {
            'S' => {
                origin = pos.into();
            }
            '^' => {
                splitters.push(pos.into());
            }
            _ => {}
        }
    }

    // Leak the finished splitter collection into a static reference we can
    // use with the memoizer.
    let splitters = splitters.leak();

    let mut beams = HashSet::from_iter([origin.x]);

    let mut splits = 0;
    for y in origin.y..bounds.height() {
        let mut new_beams = beams.clone();
        for &x in &beams {
            if splitters.contains(&ivec2(x, y)) {
                splits += 1;
                new_beams.remove(&x);
                new_beams.insert(x - 1);
                new_beams.insert(x + 1);
            }
        }

        beams = new_beams;
    }

    println!("{splits}");

    println!("{}", num_paths(bounds, splitters, origin));
}

#[memoize]
fn num_paths(
    bounds: Rect<i32>,
    splitters: &'static [IVec2],
    pos: IVec2,
) -> usize {
    for y in pos.y..bounds.height() {
        let p = ivec2(pos.x, y);
        if splitters.contains(&p) {
            return num_paths(bounds, splitters, p - ivec2(1, 0))
                + num_paths(bounds, splitters, p + ivec2(1, 0));
        }
    }

    // No splits, just one path.
    1
}
