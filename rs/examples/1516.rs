use aoc::prelude::*;

fn main() {
    const ITEMS: [(&str, usize); 10] = [
        ("cats", 7),
        ("trees", 3),
        ("goldfish", 5),
        ("pomeranians", 3),
        ("akitas", 0),
        ("cars", 2),
        ("children", 3),
        ("perfumes", 1),
        ("samoyeds", 2),
        ("vizslas", 0),
    ];

    let mut table: Vec<[Option<usize>; 10]> = Vec::new();
    for (a, an, b, bn, c, cn) in
        parsed_stdin_lines::<(String, usize, String, usize, String, usize)>(
            r"^Sue \d+: (\S+): (\d+), (\S+): (\d+), (\S+): (\d+)$",
        )
    {
        let mut row = [None; 10];
        for (item, num) in [(a, an), (b, bn), (c, cn)] {
            row[ITEMS.iter().position(|&(x, _)| x == item).unwrap()] =
                Some(num);
        }
        table.push(row);
    }

    'aunts: for (i, row) in table.iter().enumerate() {
        for j in 0..10 {
            if matches!(row[j], Some(num) if num != ITEMS[j].1) {
                continue 'aunts;
            }
        }
        println!("{}", i + 1);
        break;
    }

    'aunts: for (i, row) in table.iter().enumerate() {
        for j in 0..2 {
            if matches!(row[j], Some(num) if num <= ITEMS[j].1) {
                continue 'aunts;
            }
        }
        for j in 2..4 {
            if matches!(row[j], Some(num) if num >= ITEMS[j].1) {
                continue 'aunts;
            }
        }
        for j in 4..10 {
            if matches!(row[j], Some(num) if num != ITEMS[j].1) {
                continue 'aunts;
            }
        }
        println!("{}", i + 1);
        break;
    }
}
