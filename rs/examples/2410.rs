use aoc::prelude::*;

fn main() {
    let (bounds, grid) = stdin_grid();
    let grid: Vec<u32> = grid
        .into_iter()
        .map(|c| c.to_digit(10).unwrap_or(99))
        .collect();

    let starts: Vec<IVec2> = grid
        .iter()
        .enumerate()
        .filter_map(|(i, c)| (*c == 0).then_some(bounds.get(i).into()))
        .collect();

    // Build full paths in Dijkstra, not just the end nodes.
    let neighbors = |p: &Vec<IVec2>| {
        let head = p[p.len() - 1];
        let a = grid[bounds.idx(head)];
        // Closure nicety
        let grid = &grid;
        let p = p.clone();
        neighbors_4(head).filter_map(move |q| {
            if bounds.contains(q) && grid[bounds.idx(q)] == a + 1 {
                let mut ret = p.clone();
                ret.push(q);
                Some(ret)
            } else {
                None
            }
        })
    };

    let mut p1 = 0;
    let mut p2 = 0;
    for p in &starts {
        let paths: Vec<_> = dijkstra_map(neighbors, &vec![*p])
            .filter_map(|(p, _)| {
                (grid[bounds.idx(p[p.len() - 1])] == 9).then_some(p)
            })
            .collect();
        // Unique endpoints.
        p1 += paths
            .iter()
            .map(|p| p[p.len() - 1])
            .collect::<HashSet<_>>()
            .len();
        // Unique paths.
        p2 += paths.len();
    }

    println!("{p1}");
    println!("{p2}");
}
