use aoc::prelude::*;

#[derive(Debug)]
enum Op {
    SwapPos(usize, usize),
    Reverse(usize, usize),
    RotRight(usize),
    RotLeft(usize),
    RotBased(char),
    SwapChar(char, char),
    Move(usize, usize),
}

use Op::*;

impl FromStr for Op {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &(s.split_whitespace().collect::<Vec<&str>>())[..] {
            ["swap", "position", n, _, _, m] => {
                Ok(SwapPos(n.parse::<usize>()?, m.parse::<usize>()?))
            }
            ["reverse", _, n, _, m] => {
                Ok(Reverse(n.parse::<usize>()?, m.parse::<usize>()?))
            }
            ["rotate", "right", n, _] => Ok(RotRight(n.parse::<usize>()?)),
            ["rotate", "left", n, _] => Ok(RotLeft(n.parse::<usize>()?)),
            ["rotate", "based", _, _, _, _, n] => {
                Ok(RotBased(n.parse::<char>()?))
            }
            ["swap", "letter", n, _, _, m] => {
                Ok(SwapChar(n.parse::<char>()?, m.parse::<char>()?))
            }
            ["move", _, n, _, _, m] => {
                Ok(Move(n.parse::<usize>()?, m.parse::<usize>()?))
            }
            _ => Err("bad input")?,
        }
    }
}

impl Op {
    fn apply(&self, reversed: bool, pw: &mut [u8]) {
        match self {
            // symmetric
            SwapPos(n, m) => pw.swap(*n, *m),
            // symmetric
            Reverse(n, m) => {
                let pw2 = pw.to_owned();
                for i in 0..=(m - n) {
                    pw[n + i] = pw2[m - i];
                }
            }
            // mirrored
            RotRight(n) => {
                if reversed {
                    pw.rotate_left(*n);
                } else {
                    pw.rotate_right(*n);
                }
            }
            // mirrored
            RotLeft(n) => {
                if reversed {
                    pw.rotate_right(*n);
                } else {
                    pw.rotate_left(*n);
                }
            }
            // asymmetric
            RotBased(a) => {
                let s = |x| (x + 1 + if x >= 4 { 1 } else { 0 }) % pw.len();
                let n = pw.iter().position(|&c| c == *a as u8).unwrap();
                if reversed {
                    // Is there a closed form way?
                    let a = (0..pw.len())
                        .find(|&m| {
                            let pos = (m + s(m)) % pw.len();
                            // Only >= 4 can map to pos 0 for len >= 2.
                            if pos == 0 && m < 4 {
                                false
                            } else {
                                pos == n
                            }
                        })
                        .unwrap();

                    pw.rotate_left(s(a));
                } else {
                    pw.rotate_right(s(n));
                }
            }
            // symmetric
            SwapChar(a, b) => {
                let a = pw.iter().position(|&c| c == *a as u8).unwrap();
                let b = pw.iter().position(|&c| c == *b as u8).unwrap();
                pw.swap(a, b)
            }
            // mirrored
            Move(n, m) => {
                let rot_right = (m < n) ^ reversed;
                let (n, m) = (n.min(m), n.max(m));
                if rot_right {
                    (&mut pw[*n..(m + 1)]).rotate_right(1);
                } else {
                    (&mut pw[*n..(m + 1)]).rotate_left(1);
                }
            }
        }
    }
}

fn main() {
    let ops: Vec<Op> = stdin_lines_as::<Op>().collect();

    let mut pw = "abcdefgh".as_bytes().to_owned();
    for op in &ops {
        op.apply(false, &mut pw);
    }
    println!("{}", String::from_utf8(pw.clone()).unwrap());

    let mut scrambled = "fbgdceah".as_bytes().to_owned();
    for op in ops.iter().rev() {
        op.apply(true, &mut scrambled);
    }
    println!("{}", String::from_utf8(scrambled).unwrap());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1621() {
        let ops = "\
swap position 4 with position 0
swap letter d with letter b
reverse positions 0 through 4
rotate left 1 step
move position 1 to position 4
move position 3 to position 0
rotate based on position of letter b
rotate based on position of letter d";
        let pw = "abcde";

        let ops: Vec<Op> =
            ops.lines().map(|o| o.parse::<Op>().unwrap()).collect();

        let mut pw = pw.as_bytes().to_owned();
        for op in &ops {
            op.apply(false, &mut pw);
        }
        assert_eq!(String::from_utf8(pw.clone()).unwrap(), "decab");

        for op in ops.iter().rev() {
            op.apply(true, &mut pw);
        }
        assert_eq!(String::from_utf8(pw).unwrap(), "abcde");
    }
}
