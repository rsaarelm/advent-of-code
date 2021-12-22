use std::collections::HashSet;

use aoc::prelude::*;

fn main() {
    let mut data: Vec<(bool, [i64; 6])> = Vec::new();
    for line in stdin_lines() {
        let is_on = line.split(' ').next().unwrap().len() == 2;
        let cub: [i64; 6] = fixed_numbers(line);
        data.push((is_on, cub));
    }

    let mut state = HashSet::new();
    for (bit, p) in &data {
        eprintln!("{:?}", (bit, (p[1]-p[0]) * (p[3]-p[2]) * (p[5]-p[4])));
        if p[1] < -50 || p[3] < -50 || p[5] < -50 || p[0] > 50 || p[2] > 50 || p[4] > 50 {
            continue;
        }
        for x in p[0]..=p[1] {
            for y in p[2]..=p[3] {
                for z in p[4]..=p[5] {
                    if x < -50 || y < -50 || z < -50 || x > 50 || y > 50 || z > 50 {
                        continue;
                    }

                    if *bit {
                        state.insert([x, y, z]);
                    } else {
                        state.remove(&[x, y, z]);
                    }
                }
            }
        }
    }

    println!("{}", state.len());
    println!("0");
}
