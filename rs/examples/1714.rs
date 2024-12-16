use aoc::prelude::*;
use bitvec::prelude::*;

mod knot_hash;
use knot_hash::knot_hash;

fn main() {
    let bounds = area(128, 128);

    let input = stdin_string();
    let inner: Vec<u8> = (0..128)
        .flat_map(|y| knot_hash(format!("{input}-{y}").as_bytes()))
        .collect();
    let buf = inner.view_bits::<Msb0>();

    println!("{}", buf.count_ones());

    // Part 2

    let mut groups = 0;

    // Cells of already encountered groups.
    let mut seen = HashSet::default();

    for p in bounds.into_iter() {
        if buf[bounds.idx(p)] && !seen.contains(&p) {
            groups += 1;
            // Flood fill the rest of the new group and mark it as seen.
            for (p, _) in bfs(
                |&p| {
                    neighbors_4(p)
                        .filter(|&p| bounds.contains(p) && buf[bounds.idx(p)])
                },
                &p,
            ) {
                seen.insert(p);
            }
        }
    }

    println!("{groups}");
}
