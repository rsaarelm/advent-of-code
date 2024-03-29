use aoc::prelude::*;
use glam::{vec3, Mat3};

fn main() {
    const VOID: char = '\0';
    debug_assert!(VOID == char::default());

    let (bounds, mut grid) = stdin_grid();
    // Extract instructions, remove them from grid data.
    let code: String = (0..bounds.width())
        .map(|x| grid[bounds.idx([x, bounds.height() - 1])])
        .filter(|c| !c.is_whitespace())
        .collect();
    let bounds = area(bounds.width(), bounds.height() - 2);

    // Turn spaces into Default chars.
    for c in grid.iter_mut() {
        if *c == ' ' {
            *c = VOID;
        }
    }

    // Assume cube faces are square, determine cube size.
    let s = f64::sqrt(
        (bounds
            .into_iter()
            .filter(|&p| grid[bounds.idx(p)] != VOID)
            .count()
            / 6) as _,
    ) as i32;

    // Start from leftmost face on top row.
    let face = ivec2(
        (0..)
            .step_by(s as usize)
            .find(|x| grid[bounds.idx([*x, 0])] != VOID)
            .unwrap(),
        0,
    );

    let mut walk = Vec::new();
    let mut acc = 0;
    for c in code.chars() {
        if let Some(n) = c.to_digit(10) {
            acc = acc * 10 + (n as i32);
        } else {
            walk.push(acc);
            acc = 0;
            match c {
                'R' => walk.push(1),
                'L' => walk.push(-1),
                _ => panic!("Bad char {:?}", c),
            }
        }
    }
    if acc > 0 {
        walk.push(acc);
    }

    // Starting pos x coord.
    let mut pos = face;
    let mut facing = 0; // Facings match DIR_4 exactly.

    for (phase, &n) in walk.iter().enumerate() {
        if phase % 2 == 1 {
            // turn
            facing = (facing + n).rem_euclid(4);
            continue;
        }

        // Otherwise it's a walk.
        let vec = DIR_4[facing as usize];
        for _ in 0..n {
            let mut p2 = pos + vec;

            while grid[bounds.idx(p2)] == VOID {
                // Walked off space, do a loop.
                p2 += vec;
                p2.x = p2.x.rem_euclid(bounds.width());
                p2.y = p2.y.rem_euclid(bounds.height());
            }

            if grid[bounds.idx(p2)] == '#' {
                break;
            } else {
                pos = p2;
            }
        }
    }

    // Assignment coordinates are +1 our coordinates.
    println!("{}", 4 * (pos.x + 1) + 1000 * (pos.y + 1) + facing);

    // Part 2

    // Project the map into 3D space, `cube_chart` maps 3D points back to map.
    let mut cube_chart = HashMap::default();

    // Set up a search that visits each cube face once and always proceeds to
    // a connected next face.
    let mut visited_faces = HashSet::from_iter([face]);
    let mut face_stack = vec![(face, Mat3::IDENTITY)];

    while let Some((face, m)) = face_stack.pop() {
        for [x, y] in area(s, s) {
            let chart_pos = face + ivec2(x, y);

            let c = grid[bounds.idx(chart_pos)];
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

            // This part is tricky, floating point artifacts can mess up even
            // cover.
            debug_assert!(!cube_chart.contains_key(&p3));
            cube_chart.insert(p3, chart_pos);
        }

        // Continue building cube faces while there are unmapped sectors.
        //
        // Multiplying the transformation matrix along chart traversal keeps
        // track of the 3D space frame.
        for dir in 0..4 {
            let f = face + DIR_4[dir] * s;
            if bounds.contains(f)
                && grid[bounds.idx(f)] != VOID
                && !visited_faces.contains(&f)
            {
                visited_faces.insert(f);
                face_stack.push((f, m * ROT_XY[dir]));
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
            debug_assert!(n == -1 || n == 1);
            dir = dir.cross(n * up);
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

            if grid[bounds.idx(cube_chart[&p2])] == '#' {
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
