use std::io::{stdin, BufRead};

fn f1((x, y): (i32, i32), &(ref cmd, a): &(String, i32)) -> (i32, i32) {
    let (dx, dy) = match cmd.as_ref() {
        "forward" => (a, 0),
        "down" => (0, a),
        "up" => (0, -a),
        _ => panic!(),
    };
    (x + dx, y + dy)
}

fn f2((x, y, z): (i32, i32, i32), &(ref cmd, a): &(String, i32)) -> (i32, i32, i32) {
    let (dx, dy, dz) = match cmd.as_ref() {
        "forward" => (a, a * z, 0),
        "down" => (0, 0, a),
        "up" => (0, 0, -a),
        _ => panic!(),
    };
    (x + dx, y + dy, z + dz)
}

fn main() {
    let moves: Vec<(String, i32)> = stdin()
        .lock()
        .lines()
        .map(|s| {
            let s = s.unwrap();
            let elts: Vec<&str> = s.split(' ').collect();
            (elts[0].to_string(), elts[1].parse::<i32>().unwrap())
        })
        .collect();

    let (x, y) = moves.iter().fold((0, 0), f1);
    println!("{}", x * y);
    let (x, y, _) = moves.iter().fold((0, 0, 0), f2);
    println!("{}", x * y);
}
