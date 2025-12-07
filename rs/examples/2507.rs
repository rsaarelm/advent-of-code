use aoc::prelude::*;
use memoize::memoize;

fn main() {
    let mut origin: IVec2 = Default::default();
    // Using a Vec instead of a set so memoizer can eat it later.
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

    println!(
        "{}",
        num_paths(bounds, splitters.clone(), origin)
    );
}

#[memoize]
fn num_paths(bounds: Rect<i32>, splitters: Vec<IVec2>, pos: IVec2) -> usize {
    for y in pos.y..bounds.height() {
        let p = ivec2(pos.x, y);
        if splitters.contains(&p) {
            return num_paths(bounds, splitters.clone(), p - ivec2(1, 0))
                + num_paths(bounds, splitters.clone(), p + ivec2(1, 0));
        }
    }

    // No splits, just one path.
    1
}
