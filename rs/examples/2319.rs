use aoc::{axis_box, prelude::*};

type Block = axis_box::AxisBox<i64, 4>;

const BLOCK_N: i64 = 4001;

#[derive(Debug)]
enum Dest {
    Accept,
    Reject,
    Send(String),
}

impl FromStr for Dest {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "A" => Dest::Accept,
            "R" => Dest::Reject,
            x => Dest::Send(x.to_owned()),
        })
    }
}

#[derive(Debug)]
enum Pred {
    Lt(usize, i64),
    Gt(usize, i64),
    True,
}

impl FromStr for Pred {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn reg(s: &str) -> usize {
            match s {
                "x" => 0,
                "m" => 1,
                "a" => 2,
                "s" => 3,
                _ => panic!(),
            }
        }

        if let Some((a, b)) = s.split_once('<') {
            Ok(Pred::Lt(reg(a), b.parse().unwrap()))
        } else if let Some((a, b)) = s.split_once('>') {
            Ok(Pred::Gt(reg(a), b.parse().unwrap()))
        } else {
            Err(())
        }
    }
}

impl Pred {
    fn carve(&self, block: &Block) -> (Block, Block) {
        // Construct intersection volumes.
        let (accept, reject) = match *self {
            Pred::Lt(i, a) => {
                let mut end = [BLOCK_N; 4];
                end[i] = a;
                let mut start = [1; 4];
                start[i] = a;
                (Block::new([1; 4], end), Block::new(start, [BLOCK_N; 4]))
            }
            Pred::Gt(i, a) => {
                let mut end = [BLOCK_N; 4];
                end[i] = a + 1;
                let mut start = [1; 4];
                start[i] = a + 1;
                (Block::new(start, [BLOCK_N; 4]), Block::new([1; 4], end))
            }
            Pred::True => {
                (Block::new([1; 4], [BLOCK_N; 4]), Default::default())
            }
        };
        (block.intersection(&accept), block.intersection(&reject))
    }
}

fn process(
    workflows: &HashMap<String, Vec<(Pred, Dest)>>,
    flow: &str,
    accepted: &mut Vec<Block>,
    block: &Block,
) {
    let mut input;
    let mut current = *block;

    for (p, d) in &workflows[flow] {
        (input, current) = p.carve(&current);
        match d {
            Dest::Accept => {
                accepted.push(input);
            }
            Dest::Reject => {}
            Dest::Send(s) => {
                process(workflows, s, accepted, &input);
            }
        }
    }
}

fn main() {
    let mut workflows = HashMap::default();
    let mut ratings: Vec<[i64; 4]> = Vec::new();

    let flow = re_parser::<(String, String)>(r"(.*)\{(.*)\}");
    for line in stdin_lines() {
        if line.trim().is_empty() {
            continue;
        }

        if line.starts_with("{x=") {
            ratings.push(fixed_numbers(line));
        } else {
            let (name, body) = flow(&line).unwrap();
            let mut rules: Vec<(Pred, Dest)> = Vec::new();
            for r in body.split(',') {
                if let Some((p, d)) = r.split_once(':') {
                    rules.push((p.parse().unwrap(), d.parse().unwrap()));
                } else {
                    rules.push((Pred::True, r.parse().unwrap()));
                }
            }

            workflows.insert(name, rules);
        }
    }

    let mut valid_volume = Vec::new();
    process(
        &workflows,
        "in",
        &mut valid_volume,
        &Block::new([1; 4], [BLOCK_N; 4]),
    );

    // P1
    println!(
        "{}",
        ratings
            .iter()
            .filter(|&&r| valid_volume.iter().any(|a| a.contains(r)))
            .map(|r| r.iter().sum::<i64>())
            .sum::<i64>()
    );

    // P2
    println!("{}", valid_volume.iter().map(|b| b.volume()).sum::<i64>());
}
