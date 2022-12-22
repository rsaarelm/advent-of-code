use std::{collections::HashMap, collections::HashSet};

use aoc::prelude::*;
use glam::{vec3, Mat3};

fn main() {
    const VOID: char = '\0';
    debug_assert!(VOID == char::default());

    let (w, h, mut grid) = stdin_grid();
    // Extract instructions, remove them from grid data.
    let code: String = grid[grid.len() - 1].iter().copied().collect();
    let h = h - 1;
    grid.truncate(grid.len() - 1);

    // Turn spaces into Default chars.
    for line in grid.iter_mut() {
        for c in line.iter_mut() {
            if *c == ' ' {
                *c = VOID;
            }
        }
    }

    // Corner cases b gone
    let grid = InfiniteGrid(grid);

    // Starting pos x coord.
    let x0 = (0..w)
        .find(|&x| grid.get(ivec2(x as i32, 0)) != VOID)
        .unwrap() as i32;

    let mut walk = Vec::new();
    let mut acc = 0;
    for c in code.chars() {
        if c.is_whitespace() {
            continue;
        }
        if let Some(n) = c.to_digit(10) {
            acc = acc * 10 + (n as i32);
        } else {
            walk.push(acc);
            acc = 0;
            if c == 'R' {
                walk.push(1);
            } else if c == 'L' {
                walk.push(-1);
            } else {
                panic!("Bad char {:?}", c);
            }
        }
    }
    if acc > 0 {
        walk.push(acc);
    }

    let mut pos = ivec2(x0, 0);

    let mut facing: i32 = 0; // Facings match DIR_4 exactly.
    for (phase, &n) in walk.iter().enumerate() {
        if phase % 2 == 1 {
            // turn
            facing += n;
            facing = facing.rem_euclid(4);
            continue;
        }

        // Otherwise it's a walk.
        let vec = DIR_4[facing as usize];
        for _ in 0..n {
            let mut p2 = pos + vec;

            while grid.get(p2) == VOID {
                // Walked off space, do a loop.
                p2 += vec;
                p2.x = p2.x.rem_euclid(w as i32);
                p2.y = p2.y.rem_euclid(h as i32);
            }

            if grid.get(p2) == '#' {
                break;
            } else {
                pos = p2;
            }
        }
    }

    // Assignment coordinates are +1 our coordinates.
    println!("{}", 4 * (pos.x + 1) + 1000 * (pos.y + 1) + facing);

    // Part 2

    // Assume cube faces are square, determine cube size.
    let s = ((area((w as i32, h as i32))
        .into_iter()
        .filter(|p| grid.get(IVec2::from(*p)) != VOID)
        .count()
        / 6) as f64)
        .sqrt() as i32;

    // Build cube topology.
    let mut skeleton: HashSet<IVec2> = area((w as i32 / s, h as i32 / s))
        .into_iter()
        .filter_map(|(x, y)| {
            (grid.get(ivec2(x * s, y * s)) != VOID).then_some(ivec2(x, y))
        })
        .collect();

    // Start from leftmost face on top row.
    let face =
        ivec2((0..).find(|x| skeleton.contains(&ivec2(*x, 0))).unwrap(), 0);
    skeleton.remove(&face);

    // Build a 3D planet surface.

    let mut planet = HashSet::new();

    // Map surface positions back to 2D chart.
    let mut cube_chart = HashMap::new();

    let mut search = vec![(face, Mat3::IDENTITY)];
    while let Some((face, m)) = search.pop() {
        for (x, y) in area((s, s)) {
            let chart_pos = face * s + ivec2(x, y);

            let c = grid.get(chart_pos);
            debug_assert!(c != VOID);

            // Project to (slightly above) unit cube surface.
            // Sample cell centers so add the 0.5s
            let mut p3 = vec3(x as f32 + 0.5, y as f32 + 0.5, -0.5);
            p3 = (p3 / s as f32) - 0.5;

            // Transform to current face;
            p3 = m * p3;

            // Project back to regular space.
            p3 += 0.5;
            p3 *= s as f32;
            // Cell center correction.
            p3 -= 0.5;

            let p3 = p3.round().as_ivec3();

            debug_assert!(!cube_chart.contains_key(&p3));
            cube_chart.insert(p3, chart_pos);

            if c == '#' {
                planet.insert(p3);
            }
        }

        // Continue building cube faces while there are unmapped sectors.
        //
        // Multiplying the transformation matrix along chart traversal keeps
        // track of the 3D space frame.
        for dir in 0..4 {
            let f = face + DIR_4[dir];
            if skeleton.contains(&f) {
                search.push((f, m * ROT_XY[dir]));
                skeleton.remove(&f);
            }
        }
    }

    // *long exhalation*

    // Start out standing on top face.
    let mut pos = ivec3(0, 0, -1);
    // Facing right
    let mut dir = ivec3(1, 0, 0);
    // With the current up vector.
    let mut up = ivec3(0, 0, -1);

    for (phase, &n) in walk.iter().enumerate() {
        if phase % 2 == 1 {
            // Cross product with up vector creates correct turns on current
            // face.
            if n == -1 {
                dir = dir.cross(-up);
            } else if n == 1 {
                dir = dir.cross(up);
            } else {
                panic!("Bad turn");
            }
            continue;
        }

        for _ in 0..n {
            let mut p2 = pos + dir;
            let mut dir2 = dir;
            let mut up2 = up;

            if !cube_chart.contains_key(&p2) {
                // We walked off the face.
                // New direction points downwards from old frame.
                dir2 = -up;
                // And the new face has the same normal as the direction we
                // were walking before.
                up2 = dir;

                // Step along the new dir to get back on surface.
                p2 += dir2;
                // Sanity check.
                debug_assert!(cube_chart.contains_key(&p2));
            }

            if grid.get(cube_chart[&p2]) == '#' {
                break;
            } else {
                pos = p2;
                dir = dir2;
                up = up2;
            }
        }
    }

    let chart_pos = cube_chart[&pos];

    // Reconstruct facing.
    let facing_vec = if let Some(&p2) = cube_chart.get(&(pos + dir)) {
        // Either next position is on chart...
        p2 - chart_pos
    } else {
        // ...or the previous one is.
        chart_pos - cube_chart[&(pos - dir)]
    };
    let facing = DIR_4.iter().position(|&p| p == facing_vec).unwrap() as i32;

    println!(
        "{}",
        4 * (chart_pos.x + 1) + 1000 * (chart_pos.y + 1) + facing
    );
}
