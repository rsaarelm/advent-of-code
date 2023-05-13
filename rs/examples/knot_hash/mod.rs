use itertools::Itertools;

fn reverse(state: &mut Vec<u32>, pos: usize, n: usize) {
    let len = state.len();
    for i in 0..(n / 2) {
        let (j, k) = ((pos + i) % len, (pos + n - 1 - i) % len);
        (state[j], state[k]) = (state[k], state[j]);
    }
}

pub fn knot_hash(input: &[u8]) -> Vec<u8> {
    let input: Vec<u8> =
        input.iter().copied().chain([17, 31, 73, 47, 23]).collect();
    let mut state: Vec<u32> = (0..=255).collect();
    let mut pos = 0;
    let mut skip = 0;
    for _ in 0..64 {
        for &n in input.iter() {
            let n = n as usize;
            reverse(&mut state, pos, n);
            pos += skip + n;
            skip += 1;
        }
    }

    state
        .into_iter()
        .chunks(16)
        .into_iter()
        .map(|c| c.reduce(|acc, e| acc ^ e).unwrap_or(0) as u8)
        .collect()
}

