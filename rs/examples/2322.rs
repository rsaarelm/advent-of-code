use aoc::prelude::*;

fn others_fall(bricks: &[Cube<i32>], i: usize) -> usize {
    let mut bricks = bricks.to_owned();
    bricks.swap_remove(i);
    let mut bricks2 = bricks.clone();
    fall(&mut bricks2);

    bricks
        .into_iter()
        .zip(bricks2)
        .filter(|(a, b)| a != b)
        .count()
}

fn step(bricks: &mut [Cube<i32>]) -> usize {
    const EARTH: Cube<i32> = Cube {
        p0: [-100, -100, -100],
        p1: [101, 101, 1],
    };

    let mut drops = 0;

    'bricks: for i in 0..bricks.len() {
        let cube = bricks[i] + ivec3(0, 0, -1);
        if cube.intersects(&EARTH) {
            continue;
        }

        for (j, other) in bricks.iter().enumerate() {
            if j == i {
                continue;
            }

            if cube.intersects(other) {
                continue 'bricks;
            }
        }

        bricks[i] = cube;
        drops += 1;
    }

    drops
}

fn fall(bricks: &mut [Cube<i32>]) {
    while step(bricks) > 0 {}
}

fn main() {
    let mut input: Vec<Cube<i32>> = Vec::new();
    for line in stdin_lines() {
        let [x1, y1, z1, x2, y2, z2] = fixed_numbers(line);
        input.push(Cube::new([x1, y1, z1], [x2 + 1, y2 + 1, z2 + 1]));
    }

    let mut bricks = input.clone();
    fall(&mut bricks);

    let mut n = 0;
    for i in 0..bricks.len() {
        let mut b = bricks.clone();
        b.swap_remove(i);
        if step(&mut b) == 0 {
            n += 1;
        }
    }

    println!("{n}");

    println!(
        "{}",
        (0..bricks.len())
            .map(|i| others_fall(&bricks, i))
            .sum::<usize>()
    );
}
