use aoc::prelude::*;

#[derive(Default)]
struct Chasm {
    walls: HashSet<IVec2>,
    sand: HashSet<IVec2>,
    max_y: i32,
    has_floor: bool,
}

impl FromStr for Chasm {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut walls = HashSet::default();
        let mut max_y = 0;
        for line in s.lines() {
            let coords: Vec<IVec2> =
                to_ivec2s(numbers(line).into_iter()).collect();

            for (start, end) in coords.iter().zip(coords.iter().skip(1)) {
                let span = *end - *start;
                let dir = span.signum();
                for c in 0..=span.abs().max_element() {
                    let p = *start + c * dir;
                    max_y = max_y.max(p.y);
                    walls.insert(p);
                }
            }
        }

        Ok(Chasm {
            walls,
            max_y,
            ..Default::default()
        })
    }
}

impl Chasm {
    pub fn is_blocked(&self, pos: IVec2) -> bool {
        self.walls.contains(&pos)
            || self.sand.contains(&pos)
            || (self.has_floor && pos.y > self.max_y + 1)
    }

    pub fn add_floor(&mut self) {
        self.has_floor = true;
    }

    pub fn clear_sand(&mut self) {
        self.sand.clear();
    }

    pub fn drop(&mut self, mut pos: IVec2) -> Option<IVec2> {
        loop {
            if self.has_floor && pos.y == self.max_y + 1 {
                // Stopped by floor if it exists.
                self.sand.insert(pos);
                return Some(pos);
            }

            if pos.y > (self.max_y + 2) {
                // Not stopped by walls or floor.
                return None;
            }

            if !self.is_blocked(pos + ivec2(0, 1)) {
                pos += ivec2(0, 1);
                continue;
            }
            if !self.is_blocked(pos + ivec2(-1, 1)) {
                pos += ivec2(-1, 1);
                continue;
            }
            if !self.is_blocked(pos + ivec2(1, 1)) {
                pos += ivec2(1, 1);
                continue;
            }
            self.sand.insert(pos);
            return Some(pos);
        }
    }
}

fn main() {
    let mut chasm: Chasm = from_stdin();

    for sand in 0.. {
        if chasm.drop(ivec2(500, 0)).is_none() {
            println!("{}", sand);
            break;
        }
    }

    chasm.clear_sand();
    chasm.add_floor();

    println!(
        "{}",
        bfs(
            |&p| [p + ivec2(0, 1), p + ivec2(-1, 1), p + ivec2(1, 1)]
                .into_iter()
                .filter(|&p| !chasm.is_blocked(p)),
            &ivec2(500, 0)
        )
        .count()
    );
}
