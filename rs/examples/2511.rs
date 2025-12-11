use aoc::prelude::*;
use memoize::memoize;

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

    // Wrap graph in a memoizing function friendly smart pointer.
    let graph = PtrId::new(graph);

    println!("{}", num_targets(graph.clone(), true, true, "you".to_owned()));
    println!("{}", num_targets(graph.clone(), false, false, "svr".to_owned()));
}

#[memoize]
fn num_targets(graph: PtrId<HashMap<String, Vec<String>>>, seen_dac: bool, seen_fft: bool, node: String) -> usize {
    if seen_dac && seen_fft && node == "out" {
        return 1;
    }

    let mut ret = 0;
    if let Some(arcs) = graph.get(&node) {
        for arc in arcs {
            ret += num_targets(
                graph.clone(),
                seen_dac || arc == "dac",
                seen_fft || arc == "fft",
                arc.to_owned(),
            );
        }
    }
    ret
}
