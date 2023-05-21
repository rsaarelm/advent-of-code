use aoc::prelude::*;

fn main() {
    let mut steps = 0;

    let mut states: Vec<Vec<(bool, i32, usize)>> = Default::default();

    // Input format is bleh, let's just hack up a crap parser.
    for line in stdin_lines() {
        let line = line.trim();
        if line.starts_with("Perform") {
            steps = numbers(line)[0];
        } else if line.starts_with("- Write") {
            if states.is_empty() || states[states.len() - 1].len() == 2 {
                states.push(Default::default());
            }
            let output = numbers::<u32>(line)[0] == 1;
            let i = states.len() - 1;
            states[i].push((output, 0, 0));
        } else if line.starts_with("- Move") {
            let i = states.len() - 1;
            let j = states[i].len() - 1;
            if line.contains("left") {
                states[i][j].1 = -1;
            } else if line.contains("right") {
                states[i][j].1 = 1;
            } else {
                panic!("Bad input");
            }
        } else if line.starts_with("- Continue") {
            let i = states.len() - 1;
            let j = states[i].len() - 1;
            let c = line
                .split_whitespace()
                .last()
                .unwrap()
                .chars()
                .next()
                .unwrap() as u8;
            states[i][j].2 = (c - b'A') as usize;
        }
    }

    let mut p = 0;
    let mut tape: HashSet<i32> = HashSet::default(); // Contain 1s only.
    let mut state = 0;

    for _ in 0..steps {
        let j = if tape.contains(&p) { 1 } else { 0 };
        let (write, mov, next) = states[state][j];
        if write {
            tape.insert(p);
        } else {
            tape.remove(&p);
        }
        p += mov;
        state = next;
    }
    println!("{}", tape.len());
}
