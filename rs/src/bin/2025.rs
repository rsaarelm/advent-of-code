use aoc::util;

fn parse(input: &str) -> (u64, u64) {
    let ret: Vec<u64> = input.lines().map(|s| s.parse().unwrap()).collect();
    (ret[0], ret[1])
}

struct Transform {
    subject_num: u64,
    val: u64,
}

impl Transform {
    fn new(subject_num: u64) -> Self {
        Transform {
            subject_num,
            val: 1,
        }
    }
}

impl Iterator for Transform {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        self.val = (self.subject_num * self.val) % 20201227;
        Some(self.val)
    }
}

/// Find loop size.
fn break_key(key: u64) -> usize {
    for (i, v) in Transform::new(7).enumerate() {
        if v == key {
            return i + 1;
        }
    }
    panic!("Unbreakable key");
}

fn run_1(input: &str) -> u64 {
    let (card, door) = parse(input);
    let card_loop = break_key(card);

    Transform::new(door).skip(card_loop - 1).next().unwrap()
}

fn main() {
    let input = util::slurp_stdin();
    println!("{}", run_1(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_break_key() {
        assert_eq!(8, break_key(5764801));
        assert_eq!(11, break_key(17807724));
    }

    #[test]
    fn test_match_loops() {
        let mut xform = Transform::new(5764801);
        for i in 0..20 {
            println!("{}: {}", i, xform.next().unwrap());
        }
        assert_eq!(
            14897079,
            Transform::new(5764801).skip(11 - 1).next().unwrap()
        );
    }
}
