use aoc::prelude::*;
use glam::ivec2;

trait Terrain {
    fn height(self) -> u32;
}

impl Terrain for char {
    fn height(self) -> u32 {
        let c = match self {
            'S' => 'a',
            'E' => 'z',
            c => c
        };

        c as u32 - 'a' as u32
    }
}

fn main() {
    let map = stdin_grid().2;

    let mut map_start = Default::default();
    let mut starts = Vec::new();
    let mut end = Default::default();

    for (y, line) in map.iter().enumerate() {
        for (x, t) in line.iter().enumerate() {
            let pos = ivec2(x as i32, y as i32);
            if t.height() == 0 {
                starts.push(ivec2(x as i32, y as i32));
            }
            if *t == 'S' {
                map_start = pos;
            } else if *t == 'E' {
                end = pos;
            }
        }
    }

    // Generate path costs backwards from the end point.
    let routes = dijkstra_map(
        |a| {
            let dest_height = map.get(a).height();
            DIR_4
                .iter()
                .filter_map(|&d| {
                    let b = a + d;
                    (map.contains(b) && map.get(b).height() + 1 >= dest_height).then_some(b)
                })
                .collect()
        },
        end,
    );

    println!("{}", routes[&map_start]);
    println!(
        "{}",
        starts
            .into_iter()
            .filter_map(|s| routes.get(&s))
            .min()
            .unwrap()
    );
}
