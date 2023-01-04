use aoc::prelude::*;

fn main() {
    let input = stdin_string();
    let mut p1_done = false;

    for i in 1.. {
        let data = format!("{input}{i}");
        let hash = bytes_to_hex(&md5sum(data.as_bytes()));

        if !p1_done && hash.starts_with("00000") {
            // Part 1
            println!("{i}");
            p1_done = true;
        }

        if hash.starts_with("000000") {
            // Part 2
            debug_assert!(p1_done); // P2 prefix includes P1 prefix.
            println!("{i}");
            break;
        }
    }
}
