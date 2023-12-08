use aoc::prelude::*;

fn period(
    dirs: &[usize],
    graph: &HashMap<String, (String, String)>,
    start: &str,
) -> usize {
    let mut key = start;
    for n in 0.. {
        key = if dirs[n % dirs.len()] == 0 {
            &graph[key].0
        } else {
            &graph[key].1
        };

        if key.ends_with('Z') {
            return n + 1;
        }
    }

    unreachable!()
}

fn main() {
    let mut dirs = Vec::new();
    let mut graph: HashMap<String, (String, String)> = HashMap::default();

    for line in stdin_lines() {
        if line.trim().is_empty() {
            continue;
        }

        if let Ok((key, a, b)) = re_parser(r"(.*) = \((.*), (.*)\)")(&line) {
            graph.insert(key, (a, b));
        } else {
            dirs = line.chars().map(|c| "LR".find(c).unwrap()).collect();
        }
    }

    // P1
    println!("{}", period(&dirs, &graph, "AAA"));

    // P2
    let periods = graph
        .keys()
        .filter(|k| k.ends_with('A'))
        .map(|k| period(&dirs, &graph, k))
        .collect::<Vec<usize>>();
    println!("{}", periods.into_iter().reduce(num_integer::lcm).unwrap());
}
