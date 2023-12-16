use aoc::prelude::*;

fn neighbors<'a>(
    bounds: &'a Rect<i32>,
    buf: &'a [char],
) -> impl Fn(&(IVec2, IVec2)) -> Vec<(IVec2, IVec2)> + 'a {
    |&(pos, dir)| {
        let mut ret = Vec::new();

        let mut push = |dir| {
            if bounds.contains(pos + dir) {
                ret.push((pos + dir, dir))
            }
        };

        let is_horizontal = dir.y == 0;
        match (buf[bounds.idx(pos)], is_horizontal) {
            ('|', true) | ('-', false) => {
                push(dir.cw());
                push(dir.ccw());
            }
            ('/', false) | ('\\', true) => {
                push(dir.cw());
            }
            ('/', _) | ('\\', _) => {
                push(dir.ccw());
            }
            _ => {
                push(dir);
            }
        }

        ret
    }
}

fn main() {
    let (bounds, buf) = stdin_grid();

    let count = |p: [i32; 2], dir: [i32; 2]| {
        dijkstra_map(neighbors(&bounds, &buf), &(p.into(), dir.into()))
            .map(|((pos, _), _)| pos)
            .collect::<HashSet<_>>()
            .len()
    };

    println!("{}", count([0, 0], [1, 0]));

    let mut p2 = 0;
    for y in 0..bounds.height() {
        p2 = p2.max(count([0, y], [1, 0]));
        p2 = p2.max(count([bounds.width() - 1, y], [-1, 0]));
    }

    for x in 0..bounds.width() {
        p2 = p2.max(count([x, 0], [0, 1]));
        p2 = p2.max(count([x, bounds.height() - 1], [0, -1]));
    }

    println!("{p2}");
}
