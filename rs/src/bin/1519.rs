use aoc::prelude::*;

fn neighbors(pairs: &[(String, String)], input: &str) -> HashSet<String> {
    pairs
        .iter()
        .flat_map(|(a, b)| {
            input.match_indices(a).map(move |(i, _)| {
                format!("{}{}{}", &input[..i], b, &input[i + a.len()..])
            })
        })
        .collect()
}

fn main() {
    let mut expansions = Vec::new();
    let mut input = String::new();

    for i in stdin_lines() {
        let halves = i.split(" => ").collect::<Vec<&str>>();
        if halves.len() == 2 {
            expansions.push((halves[0].to_string(), halves[1].to_string()));
            continue;
        }

        if !i.is_empty() {
            input = i.to_string();
            break;
        }
    }

    println!("{}", neighbors(&expansions, &input).len());

    let contractions: Vec<(String, String)> = expansions
        .iter()
        .map(|(a, b)| (b.clone(), a.clone()))
        .collect();

    let end = "e".to_string();
    let path = astar_search(
        |s| neighbors(&contractions, s).into_iter(),
        |a, b| (a.len() as f32 - b.len() as f32).abs(),
        input,
        &end,
    )
    .unwrap();
    println!("{}", path.len());
}
