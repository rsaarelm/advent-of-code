use aoc::prelude::*;

fn sweep(input: &HashMap<usize, usize>, offset: usize) -> (bool, usize) {
    let mut threat = 0;
    let mut caught = false;
    for (t, range) in input {
        let span = 2 * range - 2;
        if (t + offset) % span == 0 {
            threat += t * range;
            caught = true;
        }
    }
    (caught, threat)
}

fn main() {
    let mut input = HashMap::default();
    for line in stdin_lines() {
        let [a, b]: [usize; 2] = fixed_numbers(line);
        input.insert(a, b);
    }

    println!("{}", sweep(&input, 0).1);

    for i in 0.. {
        if !sweep(&input, i).0 {
            println!("{i}");
            break;
        }
    }
}
