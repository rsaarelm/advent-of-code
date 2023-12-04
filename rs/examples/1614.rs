use std::collections::VecDeque;

use itertools::Itertools;

use aoc::prelude::*;

const WINDOW_SIZE: usize = 1000;

struct QuintFilter<I> {
    inner: I,
    window: VecDeque<String>,
    i: usize,
    quints: [usize; 128],
}

impl<I: Iterator<Item = String>> QuintFilter<I> {
    pub fn new(inner: I) -> Self {
        let mut ret = QuintFilter {
            inner,
            window: VecDeque::new(),
            i: 0,
            quints: [0; 128],
        };

        for _ in 0..WINDOW_SIZE {
            ret.push();
        }

        ret
    }

    fn push(&mut self) {
        if let Some(key) = self.inner.next() {
            for (n, c) in key.chars().dedup_with_count() {
                if n >= 5 {
                    self.quints[c as usize] = self.i;
                }
            }

            self.i += 1;
            self.window.push_back(key);
        }
    }
}

impl<I: Iterator<Item = String>> Iterator for QuintFilter<I> {
    type Item = Option<String>;

    fn next(&mut self) -> Option<Self::Item> {
        let Some(key) = self.window.pop_front() else {
            return None;
        };

        let mut ok = false;
        for (n, c) in key.chars().dedup_with_count() {
            if n >= 3 {
                if self.quints[c as usize] + WINDOW_SIZE > self.i {
                    ok = true;
                } else {
                    break;
                }
            }
        }

        self.push();

        Some(ok.then_some(key))
    }
}

fn hashes(salt: &str, stretch: usize) -> impl Iterator<Item = String> + '_ {
    (0..).map(move |i| {
        let mut s = bytes_to_hex(&md5sum(format!("{salt}{i}").as_bytes()));
        for _ in 0..stretch {
            s = bytes_to_hex(&md5sum(s.as_bytes()));
        }
        s
    })
}

fn main() {
    let input = stdin_string();

    for stretch in [0, 2016] {
        println!(
            "{}",
            QuintFilter::new(hashes(&input, stretch))
                .enumerate()
                .flat_map(|(i, c)| c.is_some().then_some(i))
                .nth(63)
                .unwrap()
        );
    }
}
