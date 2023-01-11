use aoc::prelude::*;

fn footprint<'a>(
    i: impl Iterator<Item = &'a IVec2> + 'a,
) -> impl Iterator<Item = IVec2> + 'a {
    i.scan(ivec2(0, 0), |acc, &x| {
        let ret = Some(*acc);
        *acc += x;
        ret
    })
}

fn main() {
    let input: Vec<IVec2> = stdin_string()
        .chars()
        .map(|c| DIR_4[">v<^".find(c).unwrap()])
        .collect();

    // Part 1
    println!("{}", footprint(input.iter()).collect::<HashSet<_>>().len());

    // Part 2
    let mut total: HashSet<IVec2> = HashSet::default();
    total.extend(footprint(input.iter().step_by(2)));
    total.extend(footprint(input.iter().skip(1).step_by(2)));
    println!("{}", total.len());
}
