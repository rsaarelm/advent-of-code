use aoc::prelude::*;

fn main() {
    let mut ps = Vec::new();
    let mut vs = Vec::new();
    for [px, py, vx, vy] in stdin_lines().map(fixed_numbers) {
        ps.push(ivec2(px, py));
        vs.push(ivec2(vx, vy));
    }

    let mut bounds = area(101, 103);

    // Example override
    let is_example = ps.len() == 12;
    if is_example {
        bounds = area(11, 7);
    }

    let mut scores = HashMap::default();
    for i in 0..ps.len() {
        let p2 = bounds.mod_proj(ps[i] + 100 * vs[i]);
        if p2.x == bounds.width() / 2 || p2.y == bounds.height() / 2 {
            continue;
        }
        *scores
            .entry((p2.x * 2 / bounds.width(), p2.y * 2 / bounds.height()))
            .or_default() += 1;
    }

    println!("{}", scores.values().product::<i64>());

    // P2
    //
    // We don't know what the tree looks like, but we assume it'll be the
    // frame with the lowest Shannon entropy out of the cycle of frames.

    let orig_ps = ps.clone();
    let mut min_entropy = f64::MAX;
    let mut min_entropy_frame = 0;
    for i in 0.. {
        // Pick an arbitrary bin size of 8x8 cells and compute Shannon entropy
        // with that.
        let entropy = entropy(ps.iter().map(|&p| p / ivec2(8, 8)));

        if entropy < min_entropy {
            min_entropy = entropy;
            min_entropy_frame = i;
        }

        for i in 0..ps.len() {
            ps[i] = bounds.mod_proj(ps[i] + vs[i]);
        }

        if ps == orig_ps {
            // Cycle detected.
            break;
        }
    }

    println!("{min_entropy_frame}");
}
