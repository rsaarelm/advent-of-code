use aoc::prelude::*;
use derive_deref::{Deref, DerefMut};

#[derive(Default, Deref, DerefMut)]
struct Spiral(IVec2);

impl Iterator for Spiral {
    type Item = IVec2;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = **self;

        let r = self.x.abs().max(self.y.abs());

        if self.x == r && self.y < r && self.y > -r {
            **self += ivec2(0, -1);
        } else if self.y == -r && self.x > -r {
            **self += ivec2(-1, 0);
        } else if self.x == -r && self.y < r {
            **self += ivec2(0, 1);
        } else {
            **self += ivec2(1, 0);
        }

        Some(ret)
    }
}

#[derive(Default)]
struct Convolution {
    inner: Spiral,
    seen: HashMap<IVec2, usize>,
}

impl Iterator for Convolution {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let pos = self.inner.next()?;
        let a = neighbors_8(pos)
            .filter_map(|p| self.seen.get(&p))
            .sum::<usize>()
            .max(1);
        self.seen.insert(pos, a);
        Some(a)
    }
}

fn main() {
    let input: usize = from_stdin();

    println!("{}", Spiral::default().nth(input - 1).unwrap().taxi_len());
    println!("{}", Convolution::default().find(|&a| a > input).unwrap());
}
