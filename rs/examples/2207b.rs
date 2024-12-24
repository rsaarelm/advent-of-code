use std::sync::LazyLock;

use aoc::prelude::*;

const TOTAL_SPACE: usize = 70_000_000;
const NEEDED_SPACE: usize = 30_000_000;
const DELETION_CANDIDATE: usize = 100_000;

static FILE_ENTRY: LazyLock<ReParser<usize>> =
    LazyLock::new(|| ReParser::new(r"^(\d+) .+$"));
static SUBDIR_CD: LazyLock<ReParser<String>> =
    LazyLock::new(|| ReParser::new(r"^\$ cd (.+)$"));

#[derive(Default, Debug)]
struct Dir {
    local_size: usize,
    subdirs: Vec<Dir>,
}

impl Dir {
    fn parse(mut input: &[String]) -> Result<(Dir, &[String]), ()> {
        let mut ret = Default::default();

        loop {
            if input.is_empty() {
                return Ok((ret, &[]));
            }

            let line = &input[0];
            input = &input[1..];

            if line == "$ cd .." {
                // Pop out.
                return Ok((ret, input));
            } else if SUBDIR_CD.parse(line).is_ok() {
                // Push in.
                let (subdir, rest) = Dir::parse(input)?;
                input = rest;
                ret.subdirs.push(subdir);
            } else if line == "$ ls" || line.starts_with("dir ") {
                continue;
            } else if let Ok(s) = FILE_ENTRY.parse(line) {
                ret.local_size += s;
            } else {
                panic!("Unparsed line {:?}", line);
            }
        }
    }

    fn size(&self) -> usize {
        self.local_size + self.subdirs.iter().map(|s| s.size()).sum::<usize>()
    }

    fn iter(&self) -> impl Iterator<Item = &Dir> {
        let mut stack = vec![self];
        std::iter::from_fn(move || {
            if let Some(a) = stack.pop() {
                a.subdirs.iter().for_each(|b| stack.push(b));
                Some(a)
            } else {
                None
            }
        })
    }
}

fn main() {
    let lines: Vec<String> = stdin_lines().collect();

    // Skip the initial "cd /" that only shows up at start of input.
    let (fs, _) = Dir::parse(&lines[1..]).unwrap();

    let mut sizes: Vec<usize> = fs.iter().map(|a| a.size()).collect();
    sizes.sort();

    println!(
        "{}",
        sizes
            .iter()
            .filter(|&&s| s <= DELETION_CANDIDATE)
            .sum::<usize>()
    );

    // Largest size, sorted to the end of the size list, is the root
    // directory. It equals the total disk space used since all the
    // subdirectories are counted into it.
    let available_space = TOTAL_SPACE - sizes[sizes.len() - 1];
    debug_assert!(available_space < NEEDED_SPACE);

    let extra_space_needed = NEEDED_SPACE - available_space;
    println!(
        "{}",
        sizes.iter().find(|&&s| s >= extra_space_needed).unwrap()
    );
}
