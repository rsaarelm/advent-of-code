use aoc::prelude::*;

fn evolve(bounds: &NRange<i32, 2>, grid: &mut HashSet<[i32; 2]>) {
    let mut g2 = HashSet::default();
    for p in *bounds {
        let lit = grid.contains(&p);
        let neighbors = neighbors_8(p).filter(|q| grid.contains(q)).count();
        if !lit && neighbors == 3 {
            g2.insert(p);
        }
        if lit && neighbors == 2 || neighbors == 3 {
            g2.insert(p);
        }
    }
    *grid = g2;
}

fn rewire(grid: &mut HashSet<[i32; 2]>) {
    grid.insert([0, 0]);
    grid.insert([99, 0]);
    grid.insert([0, 99]);
    grid.insert([99, 99]);
}

fn main() {
    let bounds = area(100, 100);
    let start: HashSet<[i32; 2]> = stdin_grid_iter()
        .filter_map(|(p, c)| (c == '#').then_some(p))
        .collect();

    // Part 1
    let mut grid = start.clone();
    for _ in 0..100 {
        evolve(&bounds, &mut grid);
    }
    println!("{}", grid.len());

    // Part 2
    let mut grid = start.clone();
    rewire(&mut grid);
    for _ in 0..100 {
        evolve(&bounds, &mut grid);
        rewire(&mut grid);
    }
    println!("{}", grid.len());
}
