use aoc::prelude::*;

fn main() {
    let (bounds, grid) = stdin_grid();

    let mut p1 = 0;
    let mut p2 = 0;

    // Consume regions with floodfill.
    let mut points = HashSet::from_iter(bounds);
    while let Some(seed) = points.pop() {
        let c = grid[bounds.idx(seed)];
        let region = dijkstra_map(
            |&p| {
                neighbors_4(p)
                    .filter(|&p| bounds.contains(p) && grid[bounds.idx(p)] == c)
            },
            &seed,
        )
        .map(|(p, _)| p.into())
        .collect::<HashSet<IVec2>>();

        let area = region.len();

        // Save perimeter edges with normals in a 2x space where we have room
        // for all side walls.
        let mut perimeter = HashMap::default();
        for &p in &region {
            let u = p * ivec2(2, 2);
            for d in DIR_4 {
                if !region.contains(&(p + d)) {
                    perimeter.insert(u + d, d);
                }
            }
        }

        for p in region {
            points.remove(&p.to_array());
        }

        p1 += area * perimeter.len();

        let mut discount_perimeter = 0;
        // Only count continuing lines at their x or y minimum for the
        // discount perimeter.
        for (u, n) in &perimeter {
            debug_assert!(u.x.rem_euclid(2) != u.y.rem_euclid(2));

            if u.y.rem_euclid(2) == 1
                && perimeter.get(&(u - ivec2(2, 0))) != Some(n)
            {
                discount_perimeter += 1;
            }
            if u.x.rem_euclid(2) == 1
                && perimeter.get(&(u - ivec2(0, 2))) != Some(n)
            {
                discount_perimeter += 1;
            }
        }

        p2 += area * discount_perimeter;
    }

    println!("{p1}");
    println!("{p2}");
}
