use aoc::prelude::*;
use glam::ivec2;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Cell {
    Start,
    End,
    Hill(u32),
}

use Cell::*;

impl From<char> for Cell {
    fn from(c: char) -> Self {
        match c {
            'S' => Start,
            'E' => End,
            c => Hill(c as u32 - 'a' as u32),
        }
    }
}

impl Cell {
    pub fn height(self) -> u32 {
        match self {
            Start => 0,
            End => 'z' as u32 - 'a' as u32,
            Hill(x) => x,
        }
    }
}

fn main() {
    let map = stdin_grid_into::<Cell>().2;

    let mut map_start = Default::default();
    let mut starts = Vec::new();
    let mut end = Default::default();

    for (y, line) in map.iter().enumerate() {
        for (x, t) in line.iter().enumerate() {
            let pos = ivec2(x as i32, y as i32);
            if t.height() == 0 {
                starts.push(ivec2(x as i32, y as i32));
            }
            if *t == Start {
                map_start = pos;
            } else if *t == End {
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
