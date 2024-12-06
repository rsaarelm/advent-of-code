use aoc::prelude::*;

// return new dir
fn step(pipe: &HashMap<IVec2, char>, pos: IVec2, dir: IVec2) -> Option<IVec2> {
    match (dir4(dir), pipe.get(&(pos + dir)).copied().unwrap_or(' ')) {
        (RIGHT, '-') => Some(DIR_4[RIGHT]),
        (RIGHT, '7') => Some(DIR_4[DOWN]),
        (RIGHT, 'J') => Some(DIR_4[UP]),

        (DOWN, '|') => Some(DIR_4[DOWN]),
        (DOWN, 'L') => Some(DIR_4[RIGHT]),
        (DOWN, 'J') => Some(DIR_4[LEFT]),

        (LEFT, '-') => Some(DIR_4[LEFT]),
        (LEFT, 'L') => Some(DIR_4[UP]),
        (LEFT, 'F') => Some(DIR_4[DOWN]),

        (UP, '|') => Some(DIR_4[UP]),
        (UP, '7') => Some(DIR_4[LEFT]),
        (UP, 'F') => Some(DIR_4[RIGHT]),

        _ => None,
    }
}

fn main() {
    let mut input: HashMap<IVec2, char> = stdin_grid_iter(&mut Rect::default())
        .map(|(p, c)| (p.into(), c))
        .collect();

    let start = input
        .iter()
        .find_map(|(p, c)| (*c == 'S').then_some(*p))
        .unwrap();

    let mut track = HashSet::default();

    let mut dir = DIR_4
        .iter()
        .find(|&&d| step(&input, start, d).is_some())
        .copied()
        .unwrap();

    // Save the two directions connecting track to start cell.
    let dir0 = dir;
    let mut dir1 = -dir;

    let mut p = start;
    track.insert(p);
    let mut n = 1;
    while (p + dir) != start {
        let d2 = step(&input, p, dir).unwrap();
        p += dir;
        track.insert(p);
        dir = d2;
        dir1 = -dir;
        n += 1;
    }

    println!("{}", n / 2);

    // Repair the 'S' into a part of the track.
    let new_s = match (1 << dir4(dir0)) | (1 << dir4(dir1)) {
        0b0011 => 'F',
        0b0101 => '-',
        0b1001 => 'L',
        0b0110 => '7',
        0b1010 => '-',
        0b1100 => 'J',
        x => panic!("{x}"),
    };
    input.insert(start, new_s);

    let mut enclosed = 0;
    for y in 0..input.keys().map(|p| p.y).max().unwrap() {
        // How many times have we fully crossed the track along this scanline.
        let mut crossings = 0;

        // Which Y-direction a side of the track we're moving along came from.
        let mut side_turn = 0;

        for x in 0.. {
            let p = ivec2(x, y);
            if !input.contains_key(&p) {
                break;
            }

            if track.contains(&p) {
                match input[&p] {
                    '|' => crossings += 1,
                    'L' => side_turn = -1,
                    'F' => side_turn = 1,
                    '7' if side_turn == -1 => crossings += 1,
                    'J' if side_turn == 1 => crossings += 1,
                    _ => {}
                }
            } else {
                // Secret Technique: Crossing-Parity Interior Determination
                if crossings % 2 == 1 {
                    enclosed += 1;
                }
            }
        }
    }

    println!("{enclosed}");
}
