use aoc::prelude::*;

#[derive(Copy, Clone)]
enum State {
    Weakened,
    Infected,
    Flagged,
}

use State::*;

fn main() {
    let mut map = HashMap::default();

    let (mut max_x, mut max_y) = (0, 0);
    for ([x, y], c) in stdin_grid_iter() {
        max_x = max_x.max(x);
        max_y = max_y.max(y);
        if c == '#' {
            map.insert([x, y], Infected);
        }
    }

    // P1

    let mut infected = map.clone();
    let mut pos = ivec2(max_x / 2, max_y / 2);
    let mut dir = UP;

    let mut infects = 0;
    for _ in 0..10_000 {
        let key = pos.into();
        if infected.contains_key(&key) {
            infected.remove(&key);
            dir = (dir + 1) % 4;
        } else {
            infected.insert(key, Infected);
            infects += 1;
            dir = (dir + 3) % 4;
        }
        pos += DIR_4[dir];
    }

    println!("{infects}");

    // P2

    let mut infected = map.clone();
    let mut pos = ivec2(max_x / 2, max_y / 2);
    let mut dir = UP;

    let mut infects = 0;
    for _ in 0..10_000_000 {
        let key = pos.into();
        match infected.get(&key).copied() {
            Some(Weakened) => {
                infected.insert(key, Infected);
                infects += 1;
            }
            Some(Infected) =>{
                dir = (dir + 1) % 4;
                infected.insert(key, Flagged);
            }
            Some(Flagged) => {
                dir = (dir + 2) % 4;
                infected.remove(&key);
            }
            None => {
                dir = (dir + 3) % 4;
                infected.insert(key, Weakened);
            }
        }
        pos += DIR_4[dir];
    }

    println!("{infects}");
}
