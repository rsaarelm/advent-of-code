use aoc::prelude::*;

fn f1((x, y): (i32, i32), &(ref cmd, a): &(String, i32)) -> (i32, i32) {
    let (dx, dy) = match cmd.as_ref() {
        "forward" => (a, 0),
        "down" => (0, a),
        "up" => (0, -a),
        _ => panic!(),
    };
    (x + dx, y + dy)
}

fn f2(
    (x, y, z): (i32, i32, i32),
    &(ref cmd, a): &(String, i32),
) -> (i32, i32, i32) {
    let (dx, dy, dz) = match cmd.as_ref() {
        "forward" => (a, a * z, 0),
        "down" => (0, 0, a),
        "up" => (0, 0, -a),
        _ => panic!(),
    };
    (x + dx, y + dy, z + dz)
}

fn main() {
    let moves: Vec<(String, i32)> = stdin_lines()
        .map(|s| match s.split(' ').collect::<Vec<&str>>().as_slice() {
            [dir, n] => (dir.to_string(), n.parse::<i32>().unwrap()),
            _ => panic!(),
        })
        .collect();

    let (x, y) = moves.iter().fold((0, 0), f1);
    println!("{}", x * y);
    let (x, y, _) = moves.iter().fold((0, 0, 0), f2);
    println!("{}", x * y);
}
