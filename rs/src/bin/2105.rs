use aoc::prelude::*;

type Line = [i32; 4];

fn orthogonal(line: &Line) -> bool {
    let [x1, y1, x2, y2] = line;
    x1 == x2 || y1 == y2
}

fn points(line: &Line) -> impl Iterator<Item = (i32, i32)> {
    let [x1, y1, x2, y2] = *line;
    let (dx, dy) = (x2 - x1, y2 - y1);
    let len = dx.abs().max(dy.abs());
    let (dx, dy) = (dx / len, dy / len);

    (0..=len).map(move |r| (x1 + dx * r, y1 + dy * r))
}

fn intersections<'a>(lines: impl Iterator<Item = &'a Line>) -> usize {
    let mut hist = HashMap::default();
    for line in lines {
        for point in points(line) {
            let p = hist.entry(point).or_insert(0);
            *p += 1;
        }
    }

    hist.into_iter().filter(|(_, n)| *n > 1).count()
}

fn main() {
    let lines: Vec<Line> = stdin_lines().map(fixed_numbers).collect();

    println!("{}", intersections(lines.iter().filter(|a| orthogonal(a))));
    println!("{}", intersections(lines.iter()));
}
