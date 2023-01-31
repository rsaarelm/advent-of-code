use std::fmt;

use aoc::prelude::*;

#[derive(Clone, Debug)]
struct Stacks(Vec<Vec<char>>);

impl FromStr for Stacks {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Example line: [A] [B]     [D]
        // Skip 1st char, read every 4th char after that for values.

        let mut stack_rows: Vec<Vec<char>> = Vec::new();
        let mut stack_count = 0;

        for line in s.lines() {
            if line.trim().is_empty() {
                // We've hit the end of stack layout input.

                // The last line was the stack numbering, throw that out.
                // It's guaranteed to list all stacks, so use it to grab the
                // count.
                stack_count = stack_rows.pop().unwrap().len();
                break;
            }
            stack_rows.push(line.chars().skip(1).step_by(4).collect());
        }

        // Build the columnar stacks from the parsed rows.
        let mut stacks: Vec<Vec<char>> = vec![Vec::new(); stack_count];
        for row in stack_rows.into_iter().rev() {
            for (i, c) in row.into_iter().enumerate() {
                if c != ' ' {
                    stacks[i].push(c);
                }
            }
        }

        Ok(Stacks(stacks))
    }
}

impl fmt::Display for Stacks {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for stack in &self.0 {
            write!(f, "{}", stack[stack.len() - 1])?;
        }
        Ok(())
    }
}

impl Stacks {
    fn move_1(&mut self, n: usize, from: usize, to: usize) {
        for _ in 0..n {
            let a = self.0[from - 1].pop().unwrap();
            self.0[to - 1].push(a);
        }
    }

    fn move_2(&mut self, n: usize, from: usize, to: usize) {
        let from = from - 1;
        let to = to - 1;

        let new_len = self.0[from].len() - n;

        for i in new_len..(self.0[from].len()) {
            let a = self.0[from][i];
            self.0[to].push(a);
        }

        self.0[from].truncate(new_len);
    }
}

fn main() {
    let input = stdin_string();

    let mut stacks_1: Stacks = input.as_str().parse().unwrap();
    let mut stacks_2 = stacks_1.clone();

    let move_parser =
        re_parser::<(usize, usize, usize)>(r"^move (\d+) from (\d+) to (\d+)$");
    let moves: Vec<(usize, usize, usize)> = input
        .lines()
        .filter_map(|line| move_parser(line).ok())
        .collect();

    for (n, from, to) in &moves {
        stacks_1.move_1(*n, *from, *to);
        stacks_2.move_2(*n, *from, *to);
    }

    println!("{}", stacks_1);
    println!("{}", stacks_2);
}
