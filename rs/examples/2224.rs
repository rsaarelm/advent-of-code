use aoc::prelude::*;
use bitvec::prelude::*;

#[derive(Debug)]
struct Valley {
    blizzards: BitVec,
    bounds: NRange<i32, 3>,
}

impl Valley {
    fn start(&self) -> IVec2 {
        ivec2(0, -1)
    }

    fn end(&self) -> IVec2 {
        IVec3::from(self.bounds.max()).xy() - ivec2(1, 0)
    }

    pub fn is_open(&self, ts: IVec3) -> bool {
        let p = self.bounds.mod_proj(ts);
        ts.xy() == self.start()
            || ts.xy() == self.end()
            || self.bounds.contains(ts * ivec3(1, 1, 0))
                && !self.blizzards[self.bounds.index_of(p)]
    }

    /// Search using 3D space-time coordinates.
    fn neighbors(&self, ts: IVec3) -> impl Iterator<Item = IVec3> + '_ {
        DIR_4
            .iter()
            .copied()
            .map(|d| d.extend(1))
            .chain(Some(ivec3(0, 0, 1)))
            .map(move |d| ts + d)
            .filter(|&p| self.is_open(p))
    }
}

impl FromStr for Valley {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (mut x2, mut y2) = (0, 0);

        let mut seeds = HashMap::default();

        for (y, line) in s.lines().enumerate() {
            // Make starting point zero for modular arithmetic of moving
            // blizzards.
            let y = y as i32 - 1;
            if y == -1 {
                // Top wall
                continue;
            } else if line.starts_with("##") {
                y2 = y;
                // Bottom wall.
                break;
            }
            for (x, c) in line.chars().enumerate() {
                let x = x as i32 - 1;
                if x == -1 {
                    // Left wall.
                    debug_assert!(c == '#');
                    continue;
                }
                if c == '#' {
                    // Right wall.
                    debug_assert!(x2 == 0 || x2 == x);
                    x2 = x;
                    break;
                }

                if let Some(i) = ">v<^".find(c) {
                    seeds.insert(ivec2(x, y), DIR_4[i]);
                }
            }
        }

        let bounds = volume([x2, y2, x2 * y2 / num_integer::gcd(x2, y2)]);
        let mut blizzards = bitvec![0; bounds.volume() as usize];

        for z in 0..bounds.depth() {
            for (p, v) in &seeds {
                let p = bounds.mod_proj((*p + *v * z).extend(z));
                blizzards.set(bounds.index_of(p), true);
            }
        }

        Ok(Valley { blizzards, bounds })
    }
}

fn main() {
    let valley: Valley = stdin_string().parse().unwrap();
    let mut t = 0;

    t +=
        dijkstra_map(|&p| valley.neighbors(p), valley.start().extend(t as i32))
            .find_map(|(p, t)| (p.xy() == valley.end()).then_some(t))
            .unwrap();
    println!("{t}");

    t += dijkstra_map(|&p| valley.neighbors(p), valley.end().extend(t as i32))
        .find_map(|(p, t)| (p.xy() == valley.start()).then_some(t))
        .unwrap();
    t +=
        dijkstra_map(|&p| valley.neighbors(p), valley.start().extend(t as i32))
            .find_map(|(p, t)| (p.xy() == valley.end()).then_some(t))
            .unwrap();
    println!("{t}");
}
