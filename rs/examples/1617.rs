use aoc::prelude::*;

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct State(String);

const DIRS: &str = "RDLU";
const DOORS: [usize; 4] = [3, 1, 2, 0];

impl State {
    fn pos(&self) -> IVec2 {
        // Assumes no `dirs` chars are in the initial input.
        self.0
            .chars()
            .map(|c| DIRS.find(c).map(|d| DIR_4[d]).unwrap_or(IVec2::default()))
            .sum::<IVec2>()
    }

    fn neighbors(&self) -> Vec<State> {
        let mut ret = Vec::new();
        let bounds = area(4, 4);
        let pos = self.pos();

        // No backing out of the final room.
        if pos == ivec2(3, 3) {
            return ret;
        }

        let hash = bytes_to_hex(&md5sum(self.0.as_bytes()));
        for i in 0..4 {
            if !bounds.contains(pos + DIR_4[i])
                || hash.as_bytes()[DOORS[i]] < b'b'
            {
                continue;
            }

            ret.push(State(format!(
                "{}{}",
                self.0,
                DIRS.as_bytes()[i] as char
            )));
        }

        ret
    }

    fn heuristic(&self) -> usize {
        (self.pos() - ivec2(3, 3)).taxi_len() as usize
    }
}

fn main() {
    let input = State(stdin_string());

    // Part 1
    let path = astar_search(&input, State::neighbors, State::heuristic, |c| {
        c.heuristic() == 0
    })
    .unwrap();
    println!("{}", &path[path.len() - 1].0[input.0.len()..]);

    // Part 2
    println!(
        "{}",
        dijkstra_map(State::neighbors, &input)
            .filter_map(|(t, n)| (t.heuristic() == 0).then_some(n))
            .max()
            .unwrap()
    );
}
