use std::collections::BTreeMap;

use aoc::prelude::*;

fn main() {
    let parser = re_parser::<(String, usize, String)>(
        r"^Valve (.*) has flow rate=(\d+); tunnels? leads? to valves? (.*)$",
    );
    let mut input = Vec::new();
    for line in stdin_lines() {
        let (valve, rate, outs) = parser(&line).unwrap();
        let outs: Vec<String> = outs.split(", ").map(|a| a.to_owned()).collect();

        input.push((valve, rate, outs));
    }

    // Rates of live valves.
    let raw_rates: BTreeMap<_, _> = input
        .iter()
        .filter_map(|(valve, rate, _)| (*rate > 0).then_some((valve.clone(), *rate)))
        .collect();

    // Links to other valves.
    let raw_links: BTreeMap<String, Vec<String>> = input
        .clone()
        .into_iter()
        .map(|(valve, _, links)| (valve, links))
        .collect();

    let mut indices = vec!["AA".to_string()];
    indices.extend(raw_rates.keys().cloned());

    // Sort to put biggest nodes first, most promising to explore.
    indices[1..].sort_by_key(|x| -(raw_rates[x] as i32));

    let mut dist = vec![vec![0; indices.len()]; indices.len()];
    for node in raw_rates.keys().chain(Some(&"AA".to_string())) {
        let n_i = indices.iter().position(|a| a == node).unwrap();
        for (p, d) in dijkstra_map(|node| raw_links[node].iter().cloned(), node.clone()) {
            if raw_rates.contains_key(&p) {
                let p_i = indices.iter().position(|a| a == &p).unwrap();
                dist[n_i][p_i] = d;
                dist[p_i][n_i] = d;
            }
        }
    }

    let mut rates = vec![0usize; indices.len()];
    for (node, rate) in &raw_rates {
        rates[indices.iter().position(|a| a == node).unwrap()] = *rate;
    }

    for (num_mobs, total_time) in [(1, 30), (2, 26)] {
        let mut perm: Vec<usize> = (1..indices.len()).collect();

        let mut max_steam = 0;

        loop {
            let mut time = vec![total_time; num_mobs];
            let mut pos = vec![0; num_mobs];

            let mut mob = 0;

            let mut steam = 0;

            // How far up the permutation did we handle rooms. Next
            // permutation has to change the prefix, can throw out everything
            // after that.
            let mut handled_prefix = perm.len();

            // Alternate between mobs when running the permutation.
            for (i, &p) in perm.iter().enumerate() {
                let mut d = dist[pos[mob]][p];

                // Current mob can't handle the next room.
                if d + 1 > time[mob] {
                    // FIXME: Not handling cases with more than 2 mobs.
                    mob = (mob + 1) % num_mobs;
                    d = dist[pos[mob]][p];

                    if d + 1 > time[mob] {
                        // Other mob can't handle it either, make note of how
                        // far the permutation was handled and bail out.
                        handled_prefix = i + 1;
                        break;
                    }
                }

                time[mob] -= d + 1;
                pos[mob] = p;
                steam += time[mob] * rates[p];

                // Cycle mob.
                mob = (mob + 1) % num_mobs;
            }

            max_steam = max_steam.max(steam);

            if !next_prefix_permutation(&mut perm, handled_prefix) {
                break;
            }
        }

        println!("{}", max_steam);
    }
}
