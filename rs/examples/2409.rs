use aoc::prelude::*;

#[derive(Clone)]
struct Disk {
    // [(begin, length, id)]
    files: Vec<(usize, usize, usize)>,
    // [(begin, length)]
    holes: Vec<(usize, usize)>,
}

impl FromStr for Disk {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut p = 0;
        let mut files = Vec::new();
        let mut holes = Vec::new();

        for (i, c) in s.chars().enumerate() {
            let len: usize = c.to_digit(10).unwrap() as usize;
            if i % 2 == 0 {
                files.push((p, len, i / 2));
                p += len;
            } else {
                holes.push((p, len));
                p += len;
            }
        }

        Ok(Disk { files, holes })
    }
}

impl Disk {
    pub fn defrag(&mut self, allow_partial: bool) -> &mut Self {
        for i in (0..self.files.len()).rev() {
            let (pos, mut len, id) = self.files[i];

            while let Some((p, allocated)) =
                self.malloc(len, pos, allow_partial)
            {
                len -= allocated;
                self.files.push((p, allocated, id));
            }

            self.files[i] = (pos, len, id);
        }

        self
    }

    fn malloc(
        &mut self,
        size: usize,
        max_pos: usize,
        allow_partial: bool,
    ) -> Option<(usize, usize)> {
        if size == 0 {
            return None;
        }

        for (p, s) in &mut self.holes {
            if *p > max_pos {
                return None;
            }

            if *s == 0 {
                continue;
            } else if *s >= size {
                let ret = Some((*p, size));
                *p += size;
                *s -= size;
                return ret;
            } else if allow_partial {
                let ret = Some((*p, *s));
                *s = 0;
                return ret;
            }
        }
        None
    }

    pub fn checksum(&self) -> usize {
        self.files
            .iter()
            .flat_map(|(begin, length, id)| {
                (*begin..(*begin + *length)).map(|i| i * *id)
            })
            .sum()
    }
}

fn main() {
    let disk: Disk = stdin_string().parse().unwrap();

    println!("{}", disk.clone().defrag(true).checksum());
    println!("{}", disk.clone().defrag(false).checksum());
}
