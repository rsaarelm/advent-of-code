use std::collections::BTreeSet;

use aoc::prelude::*;

fn main() {
    let input: Vec<I64Vec3> =
        stdin_lines().map(|a| fixed_numbers(a).into()).collect();
    let is_example = input.len() < 100;
    // Different behavior for example and real input.
    let num_connections = if is_example { 10 } else { 1000 };

    // Enumerate connections by ascending distance.
    let mut pairs = BTreeSet::new();
    for i in 0..input.len() {
        for j in (i + 1)..input.len() {
            let vec = input[j] - input[i];

            // This crashes with 32-bit vecs...
            let dist = vec.length_squared();

            pairs.insert((dist, i, j));
        }
    }

    // Assigned circuit numbers.
    let mut circuit_id: usize = 0;
    let mut num_circuits = input.len();

    // Map nodes to circuits.
    let mut circuits = HashMap::default();

    for (n, &(_, i, j)) in pairs.iter().enumerate() {
        // P1 answer
        if n == num_connections {
            let mut p1 = 1;
            for (_, size) in histogram(circuits.values()).take(3) {
                p1 *= size;
            }

            println!("{p1}");
        }

        let (a, b) = (
            circuits.get(&i).map(Clone::clone),
            circuits.get(&j).map(Clone::clone),
        );
        match (a, b) {
            // Form a brand new circuit.
            (None, None) => {
                circuits.insert(i, circuit_id);
                circuits.insert(j, circuit_id);
                circuit_id += 1;
                num_circuits -= 1;
            }
            // Connect unconnected node to existing circuit.
            (Some(c), None) => {
                circuits.insert(j, c);
                num_circuits -= 1;
            }
            (None, Some(c)) => {
                circuits.insert(i, c);
                num_circuits -= 1;
            }
            // No-op
            (Some(c), Some(d)) if c == d => {}
            // Merge circuit d to circuit c.
            (Some(c), Some(d)) => {
                for (_, a) in circuits.iter_mut() {
                    if *a == d {
                        *a = c;
                    }
                }
                num_circuits -= 1;
            }
        }

        // P2 answer, end of the loop.
        if num_circuits == 1 {
            println!("{}", input[i].x * input[j].x);
            break;
        }
    }
}
