use aoc::prelude::*;

fn p1(input: &str) {
    let mut disk = Vec::new();
    let mut holes = Vec::new();

    for (i, c) in input.chars().enumerate() {
        let is_file = i % 2 == 0;
        let len: usize = c.to_digit(10).unwrap() as usize;

        if is_file {
            let id = (i / 2) as i64;
            for _ in 0..len {
                disk.push(id);
            }
        } else {
            for _ in 0..len {
                holes.push(disk.len());
                disk.push(-1);
            }
        }
    }

    holes.reverse(); // For popping.

    while let Some(hole) = holes.pop() {
        while disk[disk.len() - 1] == -1 {
            disk.pop();
        }

        if hole >= disk.len() {
            break;
        }

        let n = disk.pop().unwrap();
        disk[hole] = n;
    }

    println!(
        "{}",
        disk.iter()
            .enumerate()
            .filter(|(_, b)| **b != -1)
            .map(|(a, b)| a as i64 * b)
            .sum::<i64>()
    );
}

fn p2(input: &str) {
    fn checksum(disk: &[(usize, usize, usize)]) -> usize {
        disk.iter()
            .flat_map(|(begin, length, id)| {
                (*begin..(*begin + *length)).map(|i| i * *id)
            })
            .sum()
    }

    let mut disk = Vec::new(); // [(begin, length, id)]
    let mut holes = Vec::new(); // [(begin, length)]
    let mut p = 0; // Current writing pos.

    for (i, c) in input.chars().enumerate() {
        let is_file = i % 2 == 0;
        let len: usize = c.to_digit(10).unwrap() as usize;

        if is_file {
            let id = i / 2;
            disk.push((p, len, id));
            p += len;
        } else {
            holes.push((p, len));
            p += len;
        }
    }

    // We don't need to refragment to the original disk.
    let mut refrag = Vec::new();

    'defrag: for i in (0..disk.len()).rev() {
        let (pos, len, id) = disk[i];

        for (hole_pos, hole_len) in &mut holes {
            if *hole_len >= len && *hole_pos < pos {
                refrag.push((*hole_pos, len, id));
                *hole_pos += len;
                *hole_len -= len;
                disk.swap_remove(i);
                continue 'defrag;
            }
        }
    }

    println!("{}", checksum(&disk) + checksum(&refrag));
}

fn main() {
    let input = stdin_string();
    p1(&input);
    p2(&input);
}
