use aoc::prelude::*;
use pathfinding::directed::dijkstra::dijkstra;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct State {
    pos: IVec2,
    dir: IVec2,
    momentum: i32,

    min_momentum: i32,
    max_momentum: i32,
}

impl State {
    fn p1() -> Self {
        State {
            pos: ivec2(0, 0),
            dir: ivec2(1, 0),
            momentum: -1,

            min_momentum: -1,
            max_momentum: 3,
        }
    }

    fn p2() -> Self {
        State {
            pos: ivec2(0, 0),
            dir: ivec2(1, 0),
            momentum: -1,

            min_momentum: 3,
            max_momentum: 10,
        }
    }

    fn is_completed(&self, bounds: &Rect<i32>) -> bool {
        self.pos + ivec2(1, 1) == bounds.max().into()
            && self.momentum >= self.min_momentum
    }

    fn successors(&self, bounds: &Rect<i32>, buf: &[u32]) -> Vec<(Self, u32)> {
        let mut ret = Vec::new();
        for dir in [self.dir, self.dir.cw(), self.dir.ccw()] {
            let pos = self.pos + dir;

            let momentum = if dir == self.dir {
                self.momentum + 1
            } else {
                if self.momentum >= 0 && self.momentum < self.min_momentum {
                    // Started moving, but speed too slow to turn, bail.
                    continue;
                }
                0
            };

            if bounds.contains(pos) && momentum < self.max_momentum {
                ret.push((
                    State {
                        pos,
                        dir,
                        momentum,
                        ..*self
                    },
                    buf[bounds.idx(pos)],
                ));
            }
        }

        ret
    }
}

fn main() {
    let (bounds, buf) = grid(stdin_string());
    let buf: Vec<u32> =
        buf.into_iter().map(|c| c.to_digit(10).unwrap()).collect();

    println!(
        "{}",
        dijkstra(
            &State::p1(),
            |s| s.successors(&bounds, &buf),
            |s| s.is_completed(&bounds),
        )
        .unwrap()
        .1
    );

    println!(
        "{}",
        dijkstra(
            &State::p2(),
            |s| s.successors(&bounds, &buf),
            |s| s.is_completed(&bounds),
        )
        .unwrap()
        .1
    );
}
