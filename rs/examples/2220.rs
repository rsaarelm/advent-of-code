use aoc::prelude::*;

#[derive(Clone)]
struct Permutation {
    numbers: Vec<i64>,
    indices: Vec<usize>,
}

impl FromStr for Permutation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut numbers = Vec::new();
        for line in s.lines() {
            numbers.push(line.parse().map_err(|_| ())?);
        }
        let indices = (0..numbers.len()).collect();
        Ok(Permutation { numbers, indices })
    }
}

impl std::ops::Index<usize> for Permutation {
    type Output = i64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.numbers[self.indices[index % self.indices.len()]]
    }
}

impl Permutation {
    fn mix(&mut self) {
        for (i, delta) in self.numbers.iter().enumerate() {
            // Current pos of number.
            let j = self.indices.iter().position(|&x| x == i).unwrap();
            let k = (j as i64 + delta).rem_euclid(self.numbers.len() as i64 - 1)
                as usize;

            let a = self.indices.remove(j);
            self.indices.insert(k, a);
        }
    }

    fn coords(&self) -> Option<i64> {
        let z = self.indices.iter().position(|&i| self.numbers[i] == 0)?;
        Some(
            [1000, 2000, 3000]
                .into_iter()
                .map(|c| self[c + z])
                .sum::<i64>(),
        )
    }

    fn apply_key(&mut self, key: i64) {
        for x in self.numbers.iter_mut() {
            *x *= key;
        }
    }
}

fn main() {
    let code: Permutation = from_stdin();

    let mut p1 = code.clone();
    p1.mix();
    println!("{}", p1.coords().unwrap());

    let mut p2 = code;
    p2.apply_key(811589153);
    for _ in 0..10 {
        p2.mix();
    }
    println!("{}", p2.coords().unwrap());
}
