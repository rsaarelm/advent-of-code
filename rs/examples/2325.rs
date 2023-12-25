use aoc::prelude::*;
use rand::Rng;

type Graph = IndexMap<usize, HashSet<usize>>;

fn edge(a: usize, b: usize) -> (usize, usize) {
    (a.min(b), a.max(b))
}

fn path(graph: &Graph, a: usize, b: usize) -> Option<Vec<usize>> {
    astar_search(
        &a,
        |a| graph.get(a).unwrap().iter().copied(),
        |_| 0,
        |&a| a == b,
    )
}

fn main() {
    let mut input: Graph = Default::default();
    let mut s = Interner::default();
    for line in stdin_lines() {
        let (a, bs) = line.split_once(": ").unwrap();
        let a = s.get(a);
        let bs: HashSet<usize> = bs.split(' ').map(|a| s.get(a)).collect();
        // Use .entry here since the key might have already been added by the
        // backlinks stage earlier.
        input.entry(a).or_default().extend(bs.clone());
        // Add the connections going the other way too.
        for b in bs {
            input.entry(b).or_default().insert(a);
        }
    }

    let mut edges = Vec::new();
    let mut rng = rand::thread_rng();
    // Path between random pairs, the three edges of the subgraph bridge
    // should see more traffic than other places.
    for _ in 0..1000 {
        let a = rng.gen_range(0..input.len());
        let mut b = a;
        while b == a {
            b = rng.gen_range(0..input.len());
        }
        let path = path(&input, a, b).unwrap();
        for i in 1..path.len() {
            edges.push(edge(path[i - 1], path[i]));
        }
    }

    // Knife goes in, guts come out.
    for ((a, b), _) in histogram(edges).take(3) {
        input[&a].remove(&b);
        input[&b].remove(&a);
    }

    let node = *input.iter().next().unwrap().0;
    let len_1 =
        dijkstra_map(|a| input.get(a).unwrap().iter().copied(), &node).count();
    let len_2 = input.len() - len_1;
    println!("{}", len_1 * len_2);
}
