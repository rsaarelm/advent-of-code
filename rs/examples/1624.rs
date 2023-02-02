use aoc::prelude::*;

#[derive(Clone, Default, Eq, PartialEq, Hash)]
struct State {
    pos: [i32; 2],
    seen: String,
}

impl State {
    pub fn new(pos: [i32; 2]) -> Self {
        State { pos, seen: Default::default() }
    }

    pub fn neighbors(&self, bounds: &NRange<i32, 2>, grid: &Vec<char>) -> Vec<State> {
        let mut ret = Vec::new();

        for pos in neighbors_4(self.pos).filter(|&p| bounds.contains(p)) {
            let c = grid[bounds.idx(pos)];
            if c == '#' {
                continue;
            }
            let mut seen = self.seen.clone();
            if c.is_ascii_digit() && !seen.contains(c) {
                seen.push(c);
                eprintln!("{seen}");
            }

            ret.push(State { pos, seen });
        }

        ret
    }

    pub fn seen(&self) -> usize {
        self.seen.len()
    }
}

fn main() {
    let (bounds, grid) = stdin_grid();

    let start = bounds.get(grid.iter().position(|&c| c == '0').unwrap());
    let ndigits = grid.iter().filter(|x| x.is_ascii_digit()).count();

    for (s, n) in dijkstra_map(|s| s.neighbors(&bounds, &grid), &State::new(start)) {
        if s.seen() == ndigits {
            println!("{n}");
            break;
        }
    }
}
