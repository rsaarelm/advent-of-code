use aoc::prelude::*;

trait Terrain {
    fn height(self) -> u32;
}

impl Terrain for char {
    fn height(self) -> u32 {
        let c = match self {
            'S' => 'a',
            'E' => 'z',
            c => c,
        };

        c as u32 - 'a' as u32
    }
}

fn main() {
    let (bounds, map) = stdin_grid();

    let mut map_start = Default::default();
    let mut starts = Vec::new();
    let mut end = Default::default();

    for pos in bounds {
        let pos = IVec2::from(pos);
        let t = map[bounds.idx(pos)];
        if t.height() == 0 {
            starts.push(pos);
        }
        if t == 'S' {
            map_start = pos;
        } else if t == 'E' {
            end = pos;
        }
    }

    // Generate path costs backwards from the end point.
    let routes: HashMap<IVec2, usize> = dijkstra_map(
        |&a| {
            let dest_height = map[bounds.idx(a)].height();
            // Make a reference that can be moved to the closure below without
            // consuming the main map.
            let map = &map;
            DIR_4.iter().filter_map(move |&d| {
                let b = a + d;
                (bounds.contains(b)
                    && map[bounds.idx(b)].height() + 1 >= dest_height)
                    .then_some(b)
            })
        },
        &end,
    )
    .collect();

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
