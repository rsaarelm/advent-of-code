use aoc::prelude::*;

fn main() {
    let input: Vec<Machine> = stdin_lines_as().collect();

    let p1 = input
        .iter()
        .map(|a| {
            bfs(|&x| a.neighbors(x), &Default::default())
                .find_map(|(m, dist)| (m == a.lights).then_some(dist))
                .unwrap()
        })
        .sum::<usize>();
    println!("{p1}");

    eprintln!("TODO: Tractable solution for P2");
    return;

    let p2 = input
        .iter()
        .map(|a| {
            astar_search(
                &vec![0; a.jolts.len()],
                |jolts: &Vec<usize>| a.neighbors_p2(jolts.clone()),
                |jolts: &Vec<usize>| {
                    jolts
                        .iter()
                        .zip(&a.jolts)
                        .map(|(a, b)| a.max(b) - a.min(b))
                        .sum::<usize>()
                },
                |jolts: &Vec<usize>| jolts == &a.jolts,
            )
            .unwrap()
            .len() - 1
        })
        .sum::<usize>();
    println!("{p2}");
}

#[derive(Clone, Debug, Default)]
struct Machine {
    lights: u32,
    buttons: Vec<u32>,
    jolts: Vec<usize>,
}

impl Machine {
    fn neighbors(&self, lights: u32) -> impl Iterator<Item = u32> + '_ {
        self.buttons.iter().map(move |&b| lights ^ b)
    }

    fn neighbors_p2<'a, 'b>(
        &'a self,
        jolts: Vec<usize>,
    ) -> impl Iterator<Item = Vec<usize>> + 'a {
        self.buttons.iter().filter_map(move |&b| {
            let mut boosted = jolts.clone();
            for (i, c) in boosted.iter_mut().enumerate() {
                if b & (1 << i) != 0 {
                    *c += 1;
                }
            }
            if boosted.iter().zip(self.jolts.iter()).any(|(&a, &b)| a > b) {
                None
            } else {
                Some(boosted)
            }
        })
    }
}

impl FromStr for Machine {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lights = Default::default();
        let mut buttons: Vec<u32> = Default::default();
        let mut jolts = Default::default();
        for s in s.split_whitespace() {
            // Bit mask for light pattern.
            if s.starts_with('[') {
                lights = s
                    .chars()
                    .enumerate()
                    .filter_map(|(i, c)| {
                        if c == '#' {
                            Some(1 << (i - 1))
                        } else {
                            None
                        }
                    })
                    .sum();
            } else if s.starts_with('(') {
                let mask: u32 =
                    numbers::<usize>(s).into_iter().map(|a| 1 << a).sum();
                buttons.push(mask);
            } else if s.starts_with('{') {
                jolts = numbers(s);
            } else {
                panic!()
            }
        }
        Ok(Machine {
            lights,
            buttons,
            jolts,
        })
    }
}
