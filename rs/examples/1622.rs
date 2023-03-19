use aoc::prelude::*;

#[derive(Copy, Clone, Debug)]
struct Volume {
    size: usize,
    used: usize,
}

impl Volume {
    fn avail(&self) -> usize {
        self.size - self.used
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct State {
    empty: IVec2,
    payload: IVec2,
}

impl State {
    fn new(empty: IVec2, payload: IVec2) -> Self {
        State { empty, payload }
    }

    fn neighbors(
        &self,
        bounds: &Rect<i32>,
        obstacles: &HashSet<IVec2>,
    ) -> Vec<State> {
        let mut ret = Vec::new();

        // Swap empty and data if they're adjacent.
        if (self.empty - self.payload).taxi_len() == 1 {
            ret.push(State::new(self.payload, self.empty));
        }

        // Move the empty node in any available 4 directions.
        for p in neighbors_4(self.empty) {
            if !bounds.contains(p)
                || obstacles.contains(&p)
                || p == self.payload
            {
                continue;
            }
            ret.push(State::new(p, self.payload));
        }

        ret
    }

    fn heuristic(&self) -> usize {
        self.payload.taxi_len() as usize
    }
}

fn main() {
    let mut volumes = Vec::new();

    let mut empty = IVec2::default();

    for line in stdin_lines().skip(2) {
        let [x, y, size, used, _, _]: [usize; 6] = fixed_numbers(line);
        let pos = ivec2(x as i32, y as i32);

        if used == 0 {
            empty = pos;
        }

        volumes.push((pos, Volume { size, used }));
    }

    // Part 1
    let mut pairs = 0;
    for i in 0..volumes.len() {
        let (_, a) = &volumes[i];
        if a.used == 0 {
            continue;
        }
        for j in 0..volumes.len() {
            if j == i {
                continue;
            }
            let (_, b) = &volumes[j];
            if b.avail() >= a.used {
                pairs += 1;
            }
        }
    }

    println!("{pairs}");

    // Part 2
    let bounds = Rect::from_points_inclusive(volumes.iter().map(|(p, _)| *p));
    let payload = ivec2(bounds.width() - 1, 0);

    // Eyeballing the data, the regular nodes all have <100 size, big ones are
    // 500-ish. Figure out the obstacle nodes with a hardcoded "100" limit
    // here, a more solid solution would figure out the threshold from the
    // input.
    let obstacles = HashSet::from_iter(
        volumes
            .iter()
            .filter_map(|(p, n)| (n.size > 100).then_some(p))
            .copied(),
    );

    let path = astar_search(
        &State::new(empty, payload),
        |s| s.neighbors(&bounds, &obstacles),
        State::heuristic,
        |s| s.heuristic() == 0,
    )
    .unwrap();
    println!("{}", path.len() - 1);
}
