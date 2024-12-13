use aoc::prelude::*;

const FLAT: I64Vec3 = i64vec3(1, 1, 0);

fn intersect(
    p1: I64Vec3,
    v1: I64Vec3,
    p2: I64Vec3,
    v2: I64Vec3,
) -> Option<(f64, f64)> {
    let a = p1.as_dvec3();
    let u = v1.as_dvec3();
    let b = p2.as_dvec3();
    let v = v2.as_dvec3();

    let sln =
        solve_float_linear_system(&[u.x, -v.x, u.y, -v.y], &[b.x - a.x, b.y - a.y])?;
    Some((sln[0], sln[1]))
}

fn solve(ps: &[I64Vec3], vs: &[I64Vec3], v: I64Vec3) -> Option<I64Vec3> {
    let mut ret = Vec::new();

    for i in 0..ps.len() {
        let Some((x, y)) = (0..ps.len())
            .filter(|&j| j != i)
            .find_map(|j| intersect(ps[i], vs[i] - v, ps[j], vs[j] - v))
        else {
            continue;
        };

        if x > 0.0 && y > 0.0 {
            let s =
                (ps[i].as_dvec3() + x * (vs[i] - v).as_dvec3()).as_i64vec3();
            ret.push(s);
        } else {
            return None;
        }
    }

    if ret.is_empty() {
        return None;
    }

    // There is occasional floating point weirdness so pick the most common
    // element out of the collected ones.
    let median: I64Vec3 = histogram(ret.iter().map(|&a| <[i64; 3]>::from(a)))
        .next()
        .unwrap()
        .0
        .into();

    // HACK: Need to filter out actual diverging solutions, but floating point
    // problems produce off-by-one values. so allow max +/- 1 deviation from
    // median along any axis.
    if ret
        .iter()
        .map(|&a| (a - median).abs().max_element())
        .max()
        .unwrap()
        > 1
    {
        return None;
    }

    Some(median)
}

fn main() {
    let mut ps = Vec::new();
    let mut vs = Vec::new();
    for line in stdin_lines() {
        let [x, y, z, dx, dy, dz] = fixed_numbers(line);
        ps.push(i64vec3(x, y, z));
        vs.push(i64vec3(dx, dy, dz));
    }

    let ps_flat = ps.iter().map(|&a| a * FLAT).collect::<Vec<_>>();
    let vs_flat = vs.iter().map(|&a| a * FLAT).collect::<Vec<_>>();

    let is_example = ps.len() == 5;

    let bounds = if is_example {
        Rect::new([7.0, 7.0], [27.0, 27.0])
    } else {
        Rect::new(
            [200000000000000.0, 200000000000000.0],
            [400000000000000.0, 400000000000000.0],
        )
    };

    let mut p1 = 0;

    for i in 0..ps.len() {
        for j in (i + 1)..ps.len() {
            if let Some((x, y)) = intersect(
                ps[i] * FLAT,
                vs[i] * FLAT,
                ps[j] * FLAT,
                vs[j] * FLAT,
            ) {
                let c = ps[i].as_dvec3() + x * vs[i].as_dvec3();
                if x >= 0.0 && y >= 0.0 && bounds.contains(c.truncate()) {
                    p1 += 1;
                }
            }
        }
    }

    println!("{p1}");

    const N: i64 = 300;

    for [x, y] in Rect::new([-N, -N], [N, N]) {
        // Filter out invalid 2D slices to narrow search space.
        if solve(&ps_flat, &vs_flat, i64vec3(x, y, 0)).is_none() {
            continue;
        }
        for z in -N..N {
            let v = i64vec3(x, y, z);
            if let Some(p) = solve(&ps, &vs, v) {
                println!("{}", p.x + p.y + p.z);
                return;
            }
        }
    }

    panic!("P2 not found");
}
