use std::sync::OnceLock;

use aoc::prelude::*;
use memoize::memoize;

static GRAPH: OnceLock<HashMap<String, Vec<String>>> = OnceLock::new();

fn main() {
    let mut graph: HashMap<String, Vec<String>> = Default::default();
    for line in stdin_lines() {
        let node = line
            .split_whitespace()
            .next()
            .unwrap()
            .strip_suffix(':')
            .unwrap()
            .to_owned();
        let edges = line
            .split_whitespace()
            .skip(1)
            .map(|a| a.to_owned())
            .collect();
        graph.insert(node, edges);
    }

    // Store graph in a global variable so a memoizing function can access it
    // without it being an argument.
    GRAPH.set(graph.clone()).unwrap();

    println!("{}", num_targets(true, true, "you".to_owned()));
    println!("{}", num_targets(false, false, "svr".to_owned()));
}

#[memoize]
fn num_targets(
    seen_dac: bool,
    seen_fft: bool,
    node: String,
) -> usize {
    if seen_dac && seen_fft && node == "out" {
        return 1;
    }

    let mut ret = 0;
    if let Some(arcs) = GRAPH.get().unwrap().get(&node) {
        for arc in arcs {
            ret += num_targets(
                seen_dac || arc == "dac",
                seen_fft || arc == "fft",
                arc.to_owned());
        }
    }
    ret
}
