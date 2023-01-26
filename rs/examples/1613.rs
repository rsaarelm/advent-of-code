use aoc::prelude::*;

fn main() {
    let n: i32 = from_stdin();
    let is_open = |[x, y]: &[i32; 2]| {
        *x >= 0
            && *y >= 0
            && (x * x + 3 * x + 2 * x * y + y + y * y + n).count_ones() % 2 == 0
    };

    let neighbors = |p: &[i32; 2]| neighbors_4(*p).filter(is_open);

    println!(
        "{}",
        astar_search(
            &[1, 1],
            neighbors,
            |p| (IVec2::from(*p) - ivec2(31, 39)).taxi_len(),
            |p| *p == [31, 39]
        )
        .unwrap()
        .len()
            - 1
    );

    // Part 2
    let mut locs: HashSet<[i32; 2]> = HashSet::from_iter([[1, 1]]);
    for (loc, n) in dijkstra_map(neighbors, &[1, 1]) {
        if n > 50 {
            break;
        }
        locs.insert(loc);
    }
    println!("{}", locs.len());
}
