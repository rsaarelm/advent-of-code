use aoc::prelude::*;

struct Transformer<T> {
    current: i32,
    next: Option<i32>,
    inner: T,
}

impl<T: Iterator<Item = Option<i32>>> Transformer<T> {
    pub fn new(inner: T) -> Self {
        Transformer {
            current: 1,
            next: None,
            inner,
        }
    }
}

impl<T: Iterator<Item = Option<i32>>> Iterator for Transformer<T> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(n) = self.next {
            // Update register to next value but return old value during this
            // round.
            let ret = self.current;
            self.next = None;
            self.current = n;
            Some(ret)
        } else {
            match self.inner.next() {
                // Out of input, shut down as well.
                None => None,
                // Noop, keep returning current value.
                Some(None) => Some(self.current),
                // Set up delayed add operation
                Some(Some(add)) => {
                    self.next = Some(self.current + add);
                    Some(self.current)
                }
            }
        }
    }
}

fn main() {
    let parser = re_parser::<i32>(r"^addx (.+)$");
    let input: Vec<Option<i32>> = stdin_lines().map(|line| parser(&line).ok()).collect();
    let signals: Vec<i32> = Transformer::new(input.into_iter()).collect();

    println!(
        "{}",
        (0..6)
            .map(|i| i * 40 + 20)
            // Cycle indexing starts from 1, cycle 20 = vec index 19.
            .map(|i| i as i32 * signals[i - 1])
            .sum::<i32>()
    );

    for y in 0..6 {
        for x in 0..40 {
            let pos = signals[x + y * 40];
            if (pos - x as i32).abs() <= 1 {
                eprint!("#");
            } else {
                eprint!(".");
            }
        }
        eprintln!();
    }
}
