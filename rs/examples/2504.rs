use aoc::prelude::*;

fn main() {
    let input: HashSet<IVec2> = stdin_grid_iter(&mut Default::default())
        .filter_map(|([x, y], c)| (c == '@').then_some(ivec2(x, y)))
        .collect();
    println!(
        "{}",
        open(&input).count());

    let mut remaining = input.clone();

    let mut running = true;
    while running {
        running = false;
        for p in open(&remaining).cloned().collect::<Vec<IVec2>>() {
            running = true;
            remaining.remove(&p);
        }
    }

    println!("{}", input.len() - remaining.len());
}

fn open(rolls: &HashSet<IVec2>) -> impl Iterator<Item = &'_ IVec2> + '_ {
    rolls
        .iter()
        .filter(|&&p| neighbors_8(p).filter(|q| rolls.contains(q)).count() < 4)
}
