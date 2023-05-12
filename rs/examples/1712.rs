use aoc::prelude::*;

fn main() {
    let input: Vec<Vec<usize>> = stdin_lines()
        .map(|line| {
            let mut x = numbers(line);
            x.remove(0);
            x
        })
        .collect();

    // Part 1
    println!(
        "{}",
        dijkstra_map(|&n| input[n].iter().copied(), &0).count()
    );

    // Part 2
    let mut pipes: HashSet<usize> = (0..input.len()).collect();
    let mut count = 0;
    while let Some(root) = pipes.pop() {
        for (a, _) in dijkstra_map(|&n| input[n].iter().copied(), &root) {
            pipes.remove(&a);
        }
        count += 1;
    }
    println!("{count}");
}
