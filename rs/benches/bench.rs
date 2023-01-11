#![feature(test)]
extern crate test;

use aoc::prelude::*;
use rand::{rngs::SmallRng, Rng, SeedableRng};
use test::Bencher;

struct Maze {
    bounds: NRange<i32, 2>,
    cells: Vec<bool>,
}

impl Default for Maze {
    fn default() -> Self {
        let mut rng = SmallRng::from_seed([123; 32]);

        let bounds = area(200, 100);
        let mut cells = vec![false; bounds.volume() as usize];

        for i in cells.iter_mut() {
            // Fudge up a sampling rate that generates a terrain that's still
            // traversable. RNG seed is fixed so this should work every time
            // unless the RNG algorithm changes.
            if rng.gen_range(0..10) == 0 {
                *i = true;
            }
        }

        Maze { bounds, cells }
    }
}

impl Maze {
    fn neighbors<'a>(&'a self, p: &IVec2) -> impl Iterator<Item = IVec2> + 'a {
        neighbors_4(*p).filter(|&p| {
            self.bounds.contains(p) && !self.cells[self.bounds.index_of(p)]
        })
    }

    fn start(&self) -> IVec2 {
        ivec2(0, 0)
    }

    fn end(&self) -> IVec2 {
        IVec2::from(self.bounds.max()) - ivec2(1, 1)
    }

    #[allow(dead_code)]
    fn print(&self, path: &Vec<IVec2>) {
        for y in 0..self.bounds.height() {
            for x in 0..self.bounds.width() {
                let pos = ivec2(x, y);
                let c = self.cells[self.bounds.index_of(pos)];
                let pathed = path.contains(&pos);
                assert!(!(c && pathed));
                if pathed {
                    eprint!(".");
                } else if c {
                    eprint!("#");
                } else {
                    eprint!(" ");
                }
            }
            eprintln!();
        }
    }
}

#[bench]
fn bench_astar(b: &mut Bencher) {
    let maze = Maze::default();

    b.iter(|| {
        astar_search(
            &maze.start(),
            |p| maze.neighbors(p),
            |&pos| (pos - maze.end()).taxi_len(),
            |&pos| pos == maze.end(),
        )
        .unwrap()
    });
}
