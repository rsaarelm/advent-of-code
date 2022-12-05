use aoc::prelude::*;

fn main() {
    // 1st phase, build stacks.
    // Example line: [A] [B]     [D]
    // Skip 1st char, read every 4th char after that for values.

    let mut stack_rows: Vec<Vec<char>> = Vec::new();
    let mut moves: Vec<[usize; 3]> = Vec::new();

    let mut stack_phase = true;
    let mut stack_count = 0;

    for line in stdin_lines() {
        if stack_phase {
            if line.trim().is_empty() {
                // We've hit the end of stack layout input.

                // The last line was the stack numbering, throw that out.
                // It's guaranteed to list all stacks, so use it to grab the
                // count.
                stack_count = stack_rows.pop().unwrap().len();
                // Move to next phase.
                stack_phase = false;
                continue;
            }
            stack_rows.push(line.chars().skip(1).step_by(4).collect());
        } else {
            // Collect moves for the remaining input.
            moves.push(fixed_numbers(line));
        }
    }

    // Build proper stacks.
    let mut stacks: Vec<Vec<char>> = vec![Vec::new(); stack_count];
    for row in stack_rows {
        for (i, c) in row.into_iter().enumerate() {
            if c != ' ' {
                stacks[i].push(c);
            }
        }
    }

    // We've built them upside down going down the rows, make the stack tops
    // be the upsides.
    for stack in stacks.iter_mut() {
        stack.reverse();
    }

    // Make a copy of the initial state for part 2.
    let stacks_2 = stacks.clone();

    for [n, from, to] in &moves {
        let from = *from - 1;
        let to = *to - 1;
        for _ in 0..*n {
            let a = stacks[from].pop().unwrap();
            stacks[to].push(a);
        }
    }

    for stack in &stacks {
        print!("{}", stack[stack.len() - 1]);
    }
    println!();

    let mut stacks = stacks_2;

    for [n, from, to] in &moves {
        let from = *from - 1;
        let to = *to - 1;

        let mut batch: Vec<_> = std::iter::repeat(()).take(*n)
            .map(|_| stacks[from].pop().unwrap())
            .collect();
        batch.reverse();

        stacks[to].append(&mut batch);
    }

    for stack in &stacks {
        print!("{}", stack[stack.len() - 1]);
    }
    println!();
}
