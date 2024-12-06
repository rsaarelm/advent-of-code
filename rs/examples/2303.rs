use aoc::prelude::*;

fn scan(
    map: &HashMap<IVec2, char>,
    pos: IVec2,
) -> Option<(i32, Option<IVec2>)> {
    // In the middle of number, discard.
    if map
        .get(&(pos - ivec2(1, 0)))
        .map_or(false, |c| c.is_ascii_digit())
    {
        return None;
    }

    let mut p = pos;
    let mut s = String::new();
    while let Some(c) = map.get(&p) {
        if c.is_ascii_digit() {
            s.push(*c);
        } else {
            break;
        }

        p += ivec2(1, 0);
    }

    if s.is_empty() {
        return None;
    }

    let mut gear = None;

    // Look for boundary symbol.
    let bounds = Rect::new(pos, pos + ivec2(s.len() as i32, 1)).inflate([1, 1]);

    let mut symbol_found = false;
    for (p, c) in bounds.into_iter().filter_map(|p| {
        let p = IVec2::from(p);
        map.get(&p).map(|&c| (p, c))
    }) {
        if c == '*' {
            gear = Some(p);
        }
        if !c.is_ascii_digit() && c != '.' {
            symbol_found = true;
        }
    }

    // No boundary symbol, not a parts number.
    if !symbol_found {
        return None;
    }

    Some((s.parse().unwrap(), gear))
}

fn main() {
    let cloud = stdin_grid_iter(&mut Rect::default())
        .map(|(p, c)| (IVec2::from(p), c))
        .collect::<HashMap<_, _>>();

    let parts: Vec<(i32, Option<IVec2>)> =
        cloud.keys().filter_map(|&k| scan(&cloud, k)).collect();

    println!("{:?}", parts.iter().map(|a| a.0).sum::<i32>());

    let mut gears: HashMap<IVec2, Vec<i32>> = HashMap::default();

    for (n, g) in parts.iter().filter_map(|(n, g)| g.map(|g| (n, g))) {
        gears.entry(g).or_default().push(*n);
    }

    let mut ratios = 0;
    for parts in gears.values() {
        if parts.len() > 1 {
            ratios += parts.iter().product::<i32>();
        }
    }
    println!("{ratios}");
}
