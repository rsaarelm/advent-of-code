use aoc::prelude::*;

fn solve_qe(input: &[u64], group_size: u64) -> u64 {
    // Group 1 len, Group 1 entanglement.
    let mut ret: Vec<(usize, u64)> = Vec::new();
    let mut perm: Vec<usize> = (0..input.len()).collect();
    let mut min_size = usize::MAX;
    let mut min_qe = u64::MAX;
    loop {
        let mut group = 0;
        let mut qe = 1;
        let mut limit = 0;

        for (i, &j) in perm.iter().enumerate() {
            group += input[j];
            qe *= input[j];
            if i + 1 > min_size || qe > min_qe {
                limit = i;
                break;
            }
            if group == group_size {
                min_size = min_size.min(i + 1);
                min_qe = min_qe.min(qe);
                ret.push((i + 1, qe));
                limit = i;
                break;
            }
            if group > group_size {
                limit = i;
                break;
            }
        }

        if !next_prefix_permutation(&mut perm, limit) {
            break;
        }
    }

    ret.sort();
    ret[0].1
}

fn main() {
    let mut input: Vec<u64> = stdin_lines_as().collect();
    // Bring the big values to the front, this makes permutations get to the
    // short groups fast.
    input.reverse();

    let total_weight = input.iter().sum::<u64>();
    println!("{}", solve_qe(&input, total_weight / 3));
    println!("{}", solve_qe(&input, total_weight / 4));
}
