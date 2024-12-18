use aoc::prelude::*;

fn path(mem: &[IVec2], bounds: &Rect<i32>, t: usize) -> Option<Vec<IVec2>> {
    let start = ivec2(0, 0);
    let end = IVec2::from(bounds.max()) - ivec2(1, 1);
    let blocks = mem
        .iter()
        .take(t.min(mem.len()))
        .copied()
        .collect::<HashSet<IVec2>>();

    grid_astar(&start, &end, |&pos| {
        neighbors_4(pos).filter(|&p| bounds.contains(p) && !blocks.contains(&p))
    })
}

fn main() {
    let input: Vec<IVec2> = stdin_lines()
        .map(|a| IVec2::from(fixed_numbers(a)))
        .collect();
    let bounds = Rect::from_points_inclusive(input.iter().copied());

    // Example vs main input.
    let p1_t = if input.len() == 25 { 12 } else { 1024 };

    println!("{}", path(&input, &bounds, p1_t).unwrap().len() - 1);

    // P2: Binary search for first blocking tile.
    let mut min = 0;
    let mut max = input.len();
    while min < max {
        let mid = min + (max - min) / 2;
        if path(&input, &bounds, mid + 1).is_some() {
            min = mid + 1;
        } else {
            max = mid;
        }
    }
    println!("{},{}", input[min].x, input[min].y);
}
