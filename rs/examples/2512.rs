use aoc::prelude::*;

fn main() {
    let input = stdin_string();
    let mut shapes: Vec<Vec<bool>> = Vec::new();
    let mut regions: Vec<(IVec2, Vec<usize>)> = Vec::new();
    for chunk in input.split("\n\n") {
        if chunk.contains('#') {
            let (_, buf) = grid(chunk);
            // First 3 elements are the index number, skip that.
            shapes.push(buf[3..].iter().map(|&c| c == '#').collect());
        } else {
            for line in chunk.lines() {
                let numbers = numbers(line);
                regions.push((
                    ivec2(numbers[0] as i32, numbers[1] as i32),
                    numbers[2..].to_owned(),
                ));
            }
        }
    }

    // Shenanigans.
    let volumes: Vec<usize> = shapes
        .iter()
        .map(|a| a.iter().filter(|&&x| x).count())
        .collect();

    let mut ret = 0;
    for (area, reqs) in &regions {
        let available = (area.x * area.y) as usize;
        let required: usize =
            reqs.iter().enumerate().map(|(i, a)| volumes[i] * a).sum();
        if required <= available {
            ret += 1;
        }
    }

    println!("{ret}");
}
