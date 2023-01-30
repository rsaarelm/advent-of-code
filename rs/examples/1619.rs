use aoc::prelude::*;

fn main() {
    let input: usize = from_stdin();

    // Part 1: Josephus problem hax: 0b1xx...x -> 0bxx...x1
    println!("{}", (input & !(1 << input.ilog2())) * 2 + 1);

    // Part 2: Switching to manual control.
    let mut v: Vec<usize> = (1..(input + 1)).collect();
    while v.len() > 1 {
        for i in 0.. {
            // Variable i takes a double role as counter for deleted items.
            // We don't cycle around in this loop, so we know there are always
            // i removed items before the item currently being removed,
            // subtract i from len and add i to the index, getting the 2 * i
            // along with the initial i.
            let j = 2 * i + (v.len() - i) / 2;

            // Once j goes past current vec len, we've started to loop around
            // and the assumptions will fail. Shift the vector to start from
            // current i and repack it to remove the deleted values.
            if j >= v.len() {
                v = v[i..]
                    .iter()
                    .chain(v[..i].iter())
                    .copied()
                    .filter(|&n| n > 0)
                    .collect();
                break;
            }
            v[j] = 0;
        }
    }
    println!("{}", v[0]);
}
