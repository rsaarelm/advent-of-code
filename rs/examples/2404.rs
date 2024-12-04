use aoc::prelude::*;

fn ray(
    input: &HashMap<[i32; 2], char>,
    start: IVec2,
    dir: IVec2,
    len: i32,
) -> String {
    (0..len)
        .filter_map(|i| input.get(&(start + i * dir).to_array()))
        .collect()
}

fn main() {
    let grid: HashMap<[i32; 2], char> = stdin_grid_iter().collect();
    let bounds: Rect<i32> = Rect::from_points_inclusive(grid.keys().copied());

    // Size of the input square.
    let dim = bounds.width().max(bounds.height());

    // P1

    let mut rays = Vec::new();

    // straight lines.
    for u in 0..dim {
        let s = ray(&grid, ivec2(u, 0), ivec2(0, 1), dim);
        rays.push(s.chars().rev().collect());
        rays.push(s);
        let s = ray(&grid, ivec2(0, u), ivec2(1, 0), dim);
        rays.push(s.chars().rev().collect());
        rays.push(s);
    }

    // diagonals
    for u in (-dim)..dim {
        let s = ray(&grid, ivec2(u + dim, 0), ivec2(-1, 1), dim);
        rays.push(s.chars().rev().collect());
        rays.push(s);
        let s = ray(&grid, ivec2(0, u), ivec2(1, 1), dim);
        rays.push(s.chars().rev().collect());
        rays.push(s);
    }

    println!(
        "{}",
        rays.iter()
            .map(|s| s.match_indices("XMAS").count())
            .sum::<usize>()
    );

    // P2

    let mut s = 0;
    for p in area(bounds.width() - 2, bounds.height() - 2) {
        let a = ray(&grid, p.into(), ivec2(1, 1), 3);
        let b = ray(&grid, IVec2::from(p) + ivec2(2, 0), ivec2(-1, 1), 3);
        if (a == "MAS" || a == "SAM") && (b == "MAS" || b == "SAM") {
            s += 1;
        }
    }

    println!("{s}");
}
