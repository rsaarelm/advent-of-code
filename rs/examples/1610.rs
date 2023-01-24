use std::collections::BTreeSet;

use aoc::prelude::*;

#[derive(Debug)]
enum Pass {
    Bot(usize),
    Output(usize),
}

use Pass::*;

impl Pass {
    pub fn new(name: &str, num: usize) -> Self {
        match name {
            "bot" => Bot(num),
            "output" => Output(num),
            _ => panic!("Bad name"),
        }
    }
}

fn give(
    bots: &HashMap<usize, [Pass; 2]>,
    held: &mut HashMap<usize, BTreeSet<usize>>,
    outputs: &mut HashMap<usize, usize>,
    bot: usize,
    item: usize,
) {
    let store = held.entry(bot).or_default();
    store.insert(item);
    if store.len() == 2 {
        let [x, y] = to_array(store.clone());
        let [a, b] = &bots[&bot];

        for (val, targ) in [(x, a), (y, b)] {
            match *targ {
                Bot(a) => {
                    give(bots, held, outputs, a, val);
                }
                Output(a) => {
                    outputs.insert(a, val);
                }
            }
        }
    }
}

fn main() {
    let input = stdin_string();

    // Phase 1, build full connection graph.
    let mut bots: HashMap<usize, [Pass; 2]> = HashMap::default();

    for line in input.lines() {
        match line.split_whitespace().collect::<Vec<_>>()[..] {
            [_, source, _, _, _, out_1, num_1, _, _, _, out_2, num_2] => {
                bots.insert(
                    source.parse().unwrap(),
                    [
                        Pass::new(out_1, num_1.parse().unwrap()),
                        Pass::new(out_2, num_2.parse().unwrap()),
                    ],
                );
            }
            _ => {}
        }
    }

    // Phase 2, parse inputs and propagate them through the graph.
    let mut held: HashMap<usize, BTreeSet<usize>> = HashMap::default();
    let mut outputs: HashMap<usize, usize> = HashMap::default();

    for line in input.lines() {
        match line.split_whitespace().collect::<Vec<_>>()[..] {
            [_, val, _, _, _, bot] => {
                give(
                    &bots,
                    &mut held,
                    &mut outputs,
                    bot.parse().unwrap(),
                    val.parse().unwrap(),
                );
            }
            _ => {}
        }
    }

    // Part 1
    println!(
        "{}",
        held.iter()
            .find_map(|(bot, vals)| (vals == &BTreeSet::from([17, 61]))
                .then_some(bot))
            .unwrap()
    );

    // Part 2
    println!("{}", outputs[&0] * outputs[&1] * outputs[&2]);
}
