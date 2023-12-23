use aoc::prelude::*;

/// Recognize graph vertex cells and return their outgoing connections.
fn match_vertex(
    bounds: &Rect<i32>,
    buf: &[char],
    pos: IVec2,
) -> Option<Vec<IVec2>> {
    // Start
    if pos == ivec2(1, 0) {
        return Some(vec![ivec2(1, 1)]);
    }

    // Goal
    if pos == IVec2::from(bounds.max()) - ivec2(2, 1) {
        return Some(Vec::new());
    }

    // Regular corridor.
    if neighbors_4(pos).any(|p| buf[bounds.idx(p)] == '.') {
        return None;
    }

    let mut ret = Vec::new();
    // HACK Turns out < and ^ never show up in inputs, so we just consider the
    // other two.
    if buf[bounds.idx(pos + ivec2(1, 0))] == '>' {
        ret.push(pos + ivec2(1, 0));
    }

    if buf[bounds.idx(pos + ivec2(0, 1))] == 'v' {
        ret.push(pos + ivec2(0, 1));
    }

    Some(ret)
}

/// Trace a graph edge. Returns its length and the next vertex.
fn trace(
    bounds: &Rect<i32>,
    buf: &[char],
    start_vertex: IVec2,
    edge_start: IVec2,
) -> (usize, IVec2, Vec<IVec2>) {
    let mut n = 1;
    let mut prev = start_vertex;
    let mut pos = edge_start;
    loop {
        if let Some(ns) = match_vertex(bounds, buf, pos) {
            return (n, pos, ns);
        }

        let new_pos = neighbors_4(pos)
            .find(|&p| buf[bounds.idx(p)] != '#' && p != prev)
            .unwrap();
        prev = pos;
        pos = new_pos;

        n += 1;
    }
}

fn worst_path(graph: &Vec<Vec<usize>>, mut seen: u64, i: usize) -> usize {
    if i == graph.len() - 1 {
        return 0;
    }

    seen |= 1 << i;

    let mut ret = 0;
    for (j, c) in graph[i]
        .iter()
        .enumerate()
        .filter(|(j, c)| **c > 0 && (seen & (1 << j) == 0))
    {
        ret = ret.max(c + worst_path(graph, seen, j));
    }
    ret
}

fn main() {
    // Spoiler: The map tunnels form a directed graph and the task is solving
    // the longest path problem, first for the directed and then for the
    // undirected graph.

    // First build a sparse graph from the input.

    let (bounds, buf) = grid(stdin_string());
    let end = IVec2::from(bounds.max()) - ivec2(2, 1);

    let mut ps = vec![(ivec2(1, 0), ivec2(1, 1))];

    let mut map_graph: IndexMap<IVec2, Vec<(IVec2, usize)>> =
        IndexMap::default();

    while let Some((v, p)) = ps.pop() {
        let (n, v2, ps2) = trace(&bounds, &buf, v, p);
        map_graph.entry(v).or_default().push((v2, n));
        if !map_graph.contains_key(&v2) {
            for p in ps2 {
                ps.push((v2, p));
            }
        }
    }
    map_graph.insert(end, Vec::new());

    // Then make it into a dense graph so we can throw away the points. We
    // only care about edge lengths and when the vertices are just list
    // indices we can use a u64 bitmask to track visited nodes.

    let mut graph = vec![vec![0; map_graph.len()]; map_graph.len()];
    for (i, (_, ns)) in map_graph.iter().enumerate() {
        for (j, n) in ns
            .iter()
            .map(|(p, n)| (map_graph.get_index_of(p).unwrap(), *n))
        {
            graph[i][j] = n;
        }
    }

    assert!(graph.len() <= 64);  // Too big for bitmask.

    println!("{}", worst_path(&graph, 0, 0));

    // Add reverse connections to make the graph undirected for P2.
    for i in 0..graph.len() {
        for j in (i + 1)..graph.len() {
            graph[j][i] |= graph[i][j];
            graph[i][j] |= graph[j][i];
        }
    }

    println!("{}", worst_path(&graph, 0, 0));
}
