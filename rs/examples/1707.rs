use aoc::prelude::*;

fn weight(
    ws: &HashMap<String, i32>,
    cs: &HashMap<String, Vec<String>>,
    s: &String,
) -> i32 {
    ws[s]
        + cs.get(s)
            .map_or(0, |x| x.iter().map(|a| weight(ws, cs, a)).sum::<i32>())
}

fn main() {
    let mut ws = HashMap::default();
    let mut cs: HashMap<String, Vec<String>> = HashMap::default();
    for line in stdin_lines() {
        let mut words = line.split(|c: char| !c.is_alphabetic());
        let name = words.next().unwrap().to_string();
        for child in
            words.filter_map(|c| (!c.is_empty()).then_some(c.to_string()))
        {
            cs.entry(name.clone()).or_default().push(child);
        }
        let [w] = fixed_numbers::<i32, 1>(line);
        ws.insert(name.clone(), w);
    }

    println!(
        "{}",
        ws.keys()
            .collect::<HashSet<_>>()
            .difference(
                &cs.values().flat_map(|c| c.iter()).collect::<HashSet<_>>()
            )
            .next()
            .unwrap()
    );

    let mut weight_level = i32::MAX;
    let mut fixed_weight = 0;

    for w in ws.keys() {
        if !cs.contains_key(w) {
            continue;
        }

        // Full weights.
        let cws: Vec<i32> = cs[w].iter().map(|a| weight(&ws, &cs, a)).collect();

        // Look for a weight that doesn't match the others. (This will only
        // find a value if the array has at least 3 elements.)
        let mut bad_idx = cws.len();
        for i in 0..(cws.len() - 1) {
            if cws[i] != cws[(i + 1) % cws.len()]
                && cws[i] != cws[(i + 2) % cws.len()]
            {
                bad_idx = i;
                break;
            }
        }
        if bad_idx == cws.len() {
            // No error found in this case, continue.
            continue;
        }

        // We might encounter the error multiple times, the case where total
        // weights are smallest should be the one with the mismatched weight.
        if cws[bad_idx] < weight_level {
            weight_level = cws[bad_idx];
            fixed_weight = ws[&cs[w][bad_idx]] + cws[(bad_idx + 1) % cws.len()]
                - cws[bad_idx];
        }
    }

    println!("{}", fixed_weight);
}
