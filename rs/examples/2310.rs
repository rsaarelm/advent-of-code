use aoc::prelude::*;

// return new dir
fn step(pipe: &HashMap<IVec2, char>, pos: IVec2, dir: IVec2) -> Option<IVec2> {
    // XXX: Can't pattern-match ivecs?
    match (
        <[i32; 2]>::from(dir),
        pipe.get(&(pos + dir)).copied().unwrap_or(' '),
    ) {
        ([1, 0], '-') => Some(DIR_4[RIGHT]),
        ([1, 0], '7') => Some(DIR_4[DOWN]),
        ([1, 0], 'J') => Some(DIR_4[UP]),

        ([0, 1], '|') => Some(DIR_4[DOWN]),
        ([0, 1], 'L') => Some(DIR_4[RIGHT]),
        ([0, 1], 'J') => Some(DIR_4[LEFT]),

        ([-1, 0], '-') => Some(DIR_4[LEFT]),
        ([-1, 0], 'L') => Some(DIR_4[UP]),
        ([-1, 0], 'F') => Some(DIR_4[DOWN]),

        ([0, -1], '|') => Some(DIR_4[UP]),
        ([0, -1], '7') => Some(DIR_4[LEFT]),
        ([0, -1], 'F') => Some(DIR_4[RIGHT]),

        _ => None,
    }
}

fn main() {
    let mut input: HashMap<IVec2, char> =
        stdin_grid_iter().map(|(p, c)| (p.into(), c)).collect();

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
    let new_s = match (1 << DIR_4.iter().position(|&a| a == dir0).unwrap())
        | (1 << DIR_4.iter().position(|&a| a == dir1).unwrap())
    {
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
