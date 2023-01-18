use aoc::prelude::*;

fn main() {
    let mut input = Vec::new();
    for line in stdin_lines() {
        let mut parts: Vec<String> =
            line.split('-').map(|c| c.to_string()).collect();
        let tail = parts.pop().unwrap();
        let sector: u32 = numbers(&tail)[0];
        let mut checksum = tail.split('[').skip(1).next().unwrap().to_string();
        checksum.pop();
        input.push((parts, sector, checksum));
    }

    let mut sum = 0;
    for (room, sector, checksum) in &input {
        let mut commons: String =
            histogram(room.iter().flat_map(|c| c.chars()))
                .take(5)
                .map(|(c, _)| c)
                .collect();
        if checksum == &commons {
            sum += sector;
        }
    }

    println!("{sum}");

    for (room, sector, checksum) in &input {
        let mut commons: String =
            histogram(room.iter().flat_map(|c| c.chars()))
                .take(5)
                .map(|(c, _)| c)
                .collect();
        if checksum == &commons {
            for word in room {
                let word: String = word
                    .chars()
                    .map(|c| {
                        char::from_u32(
                            ((c as u32 - 'a' as u32 + sector) % 26)
                                + 'a' as u32,
                        )
                        .unwrap()
                    })
                    .collect();
                if word == "northpole" {
                    print!("{sector}");
                    break;
                }
            }
        }
    }
}
