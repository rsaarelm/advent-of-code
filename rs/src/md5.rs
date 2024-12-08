use itertools::Itertools;

// Gratuitously roll our own md5 algorithm for the old tasks that don't stick
// to the newer spirit of not referencing algorithms outside the task
// description.
//
// This is obviously for entertainment purposes only, don't use this for any
// serious crypto code or as an example on how to implement cryptography-grade
// code.
//
// Implementation based on
// https://en.wikipedia.org/w/index.php?title=MD5&oldid=1125921801
pub fn md5sum(input: &[u8]) -> [u8; 16] {
    use std::num::Wrapping as W;

    // All values are little-endian.

    // K[i] := floor(2**32 * |sin(i + 1)|)
    #[rustfmt::skip]
    const K: [u32; 64] = [
        0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee,
        0xf57c0faf, 0x4787c62a, 0xa8304613, 0xfd469501,
        0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be,
        0x6b901122, 0xfd987193, 0xa679438e, 0x49b40821,
        0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa,
        0xd62f105d, 0x02441453, 0xd8a1e681, 0xe7d3fbc8,
        0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed,
        0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a,
        0xfffa3942, 0x8771f681, 0x6d9d6122, 0xfde5380c,
        0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70,
        0x289b7ec6, 0xeaa127fa, 0xd4ef3085, 0x04881d05,
        0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665,
        0xf4292244, 0x432aff97, 0xab9423a7, 0xfc93a039,
        0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1,
        0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1,
        0xf7537e82, 0xbd3af235, 0x2ad7d2bb, 0xeb86d391,
    ];

    // Shift amounts.
    #[rustfmt::skip]
    const S: [u32; 64] = [
        7, 12, 17, 22,  7, 12, 17, 22,  7, 12, 17, 22,  7, 12, 17, 22,
        5,  9, 14, 20,  5,  9, 14, 20,  5,  9, 14, 20,  5,  9, 14, 20,
        4, 11, 16, 23,  4, 11, 16, 23,  4, 11, 16, 23,  4, 11, 16, 23,
        6, 10, 15, 21,  6, 10, 15, 21,  6, 10, 15, 21,  6, 10, 15, 21,
    ];

    let mut a0 = 0x67452301u32;
    let mut b0 = 0xefcdab89u32;
    let mut c0 = 0x98badcfeu32;
    let mut d0 = 0x10325476u32;

    // Pad message so that it ends up 8 bytes away from a 64 byte boundary.
    // All messages must have nonzero padding.
    let padding_needed = 64 - ((input.len() + 8) % 64);

    let padded_input = input
        .iter()
        .copied()
        .chain([0x80])
        .chain(std::iter::repeat(0).take(padding_needed - 1))
        .chain(((input.len() * 8) as u64).to_le_bytes());

    for chunk in &padded_input.chunks(64) {
        // Noisy way to turn the byte chunk into 32-bit words.
        let m: [u32; 16] = chunk
            .chunks(4)
            .into_iter()
            .map(|word| {
                u32::from_le_bytes(
                    word.collect::<Vec<u8>>().try_into().unwrap(),
                )
            })
            .collect::<Vec<u32>>()
            .try_into()
            .unwrap();

        let (mut a, mut b, mut c, mut d) = (a0, b0, c0, d0);

        for i in 0..64 {
            let f: u32;
            let g: u32;
            if i < 16 {
                f = (b & c) | (!b & d);
                g = i;
            } else if i < 32 {
                f = (d & b) | (!d & c);
                g = (5 * i + 1) % 16;
            } else if i < 48 {
                f = b ^ c ^ d;
                g = (3 * i + 5) % 16;
            } else {
                f = c ^ (b | !d);
                g = (7 * i) % 16;
            }

            let i = i as usize;
            let g = g as usize;
            let f: u32 = (W(f) + W(a) + W(K[i]) + W(m[g])).0;
            (a, b, c, d) = (d, (W(b) + W(f.rotate_left(S[i]))).0, b, c);
        }

        a0 = (W(a0) + W(a)).0;
        b0 = (W(b0) + W(b)).0;
        c0 = (W(c0) + W(c)).0;
        d0 = (W(d0) + W(d)).0;
    }

    let mut ret = [0; 16];
    for (i, w) in [a0, b0, c0, d0].into_iter().enumerate() {
        for (j, b) in w.to_le_bytes().into_iter().enumerate() {
            ret[i * 4 + j] = b;
        }
    }

    ret
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::prelude::hex_to_bytes;

    #[test]
    fn test_md5sum() {
        assert_eq!(
            md5sum("The quick brown fox jumps over the lazy dog".as_bytes()),
            hex_to_bytes("9e107d9d372bb6826bd81d3542a419d6")[..]
        );

        assert_eq!(
            md5sum(
                "Lorem ipsum dolor sit amet, consectetur adipiscing elit"
                    .as_bytes()
            ),
            hex_to_bytes("fc10a08df7fafa3871166646609e1c95")[..]
        );
    }
}
