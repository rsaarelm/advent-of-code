use aoc::prelude::*;

fn main() {
    let mut bots = Vec::new();
    for line in stdin_lines() {
        let [x, y, z, r]: [i32; 4] = fixed_numbers(&line);
        bots.push((ivec3(x, y, z), r));
    }

    // Part 1

    let (center, center_range) =
        bots.iter().max_by_key(|(_, r)| r).copied().unwrap();

    let in_range = bots
        .iter()
        .filter(|(p, _)| (*p - center).taxi_len() <= center_range)
        .count();

    println!("{in_range}");

    // Part 2

    let pos = {
        todo!("FIXME: Get z3 working again");
        /*
        // Forbidden technique: SMT Solver no jutsu.

        use z3::{ast::Int, *};
        let cfg = Config::new();
        let ctx = Context::new(&cfg);
        let opt = Optimize::new(&ctx);

        let _x = Int::new_const(&ctx, "x");
        let _y = Int::new_const(&ctx, "y");
        let _z = Int::new_const(&ctx, "z");
        let (x, y, z) = (&_x, &_y, &_z);

        for (vec, r) in &bots {
            let _cx = ast::Int::from_i64(&ctx, vec.x as i64);
            let _cy = ast::Int::from_i64(&ctx, vec.y as i64);
            let _cz = ast::Int::from_i64(&ctx, vec.z as i64);
            let _r = ast::Int::from_i64(&ctx, *r as i64);
            let (cx, cy, cz, r) = (&_cx, &_cy, &_cz, &_r);

            // Use `ite` ternary operations to get absolute values.
            opt.assert_soft(
                &(x.le(cx).ite(&(cx - x), &(x - cx))
                    + y.le(cy).ite(&(cy - y), &(y - cy))
                    + z.le(cz).ite(&(cz - z), &(z - cz)))
                .le(r),
                1,
                None,
            );
        }

        let SatResult::Sat = opt.check(&[]) else {
            panic!("Solver failed");
        };
        let model = opt.get_model().unwrap();
        let (x, y, z) = (
            model.eval(x, true).unwrap().as_i64().unwrap() as i32,
            model.eval(y, true).unwrap().as_i64().unwrap() as i32,
            model.eval(z, true).unwrap().as_i64().unwrap() as i32,
        );

        ivec3(x, y, z)
        */
    };

    println!("{}", pos.taxi_len());
}
