#![feature(drain_filter)]
use aoc::prelude::*;

fn avg(data: &[u32], mask: u32) -> u32 {
    if data.iter().filter(|&x| x & mask != 0).count() as f32 >= data.len() as f32 / 2.0 {
        mask
    } else {
        0
    }
}

fn prune(data: &mut Vec<u32>, mask: u32, avg: u32) -> Option<u32> {
    data.drain_filter(|&mut x| x & mask != avg & mask);
    if data.len() == 1 {
        return Some(data[0]);
    } else {
        return None;
    }
}

fn main() {
    let mut n = 0;
    let data: Vec<u32> = stdin_lines()
        .map(|s| {
            n = n.max(s.len() as u32);
            u32::from_str_radix(&s, 2).unwrap()
        })
        .collect();

    // 1
    let commons: u32 = (0..n).map(|i| avg(&data, 1 << i)).sum();
    // Flip bits and prune to n bits.
    let uncommons: u32 = (!commons) & ((1 << n) - 1);
    println!("{}", commons * uncommons);

    // 2
    let mut parts = Vec::new();

    // Factor out the difference of avg(..) and !avg(..) into f.
    for f in [|a: u32| a, |a: u32| !a] {
        let mut remaining = data.clone();
        for i in (0..n).rev() {
            let mask = 1 << i;
            let avg = f(avg(&remaining, mask));
            if let Some(a) = prune(&mut remaining, mask, avg) {
                parts.push(a);
                break;
            }
        }
    }

    println!("{}", parts.iter().product::<u32>());
}
