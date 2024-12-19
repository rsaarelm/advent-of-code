use aoc::prelude::*;

/// Memoizing pattern counter.
struct Builder {
    towels: Vec<String>,
    cache: HashMap<String, usize>,
}

impl FromStr for Builder {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Builder {
            towels: s.split(", ").map(String::from).collect(),
            cache: Default::default(),
        })
    }
}

impl Builder {
    fn designs(&mut self, design: &str) -> usize {
        if design.is_empty() {
            return 1;
        }

        if let Some(&n) = self.cache.get(design) {
            return n;
        }

        let mut ret = 0;
        for i in 0..self.towels.len() {
            if let Some(suffix) = design.strip_prefix(&self.towels[i]) {
                ret += self.designs(suffix);
            }
        }

        self.cache.insert(design.to_owned(), ret);
        ret
    }
}

fn main() {
    let mut lines = stdin_lines();
    let mut builder: Builder = lines.next().unwrap().parse().unwrap();
    lines.next().unwrap();
    let designs: Vec<String> = lines.map(|a| a.to_string()).collect();

    println!(
        "{}",
        designs.iter().filter(|d| builder.designs(d) > 0).count()
    );
    println!(
        "{}",
        designs.iter().map(|d| builder.designs(d)).sum::<usize>()
    );
}
