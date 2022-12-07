use aoc::prelude::*;
use std::collections::{HashMap, HashSet};

const TOTAL_SPACE: usize = 70_000_000;
const NEEDED_SPACE: usize = 30_000_000;
const DELETION_CANDIDATE: usize = 100_000;

fn main() {
    // Look for lines that look like file size listings. ls commands can be
    // ignored.
    let file_entry = re_parser::<(usize,)>(r"^(\d+) .+$");
    let subdir_cd = re_parser::<(String,)>(r"^\$ cd (.+)$");

    // Total size for the subdirectory given a Vec<String> path string.
    let mut subdir_sizes = HashMap::new();

    // Check if a directory is being viewed twice, don't tally up sizes after
    // the first time. The input doesn't seem to have any repeat views so this
    // part didn't end up being necessary.
    let mut seen = HashSet::new();

    // Current path, works like a stack.
    let mut path = Vec::new();

    for line in stdin_lines() {
        if line == "$ cd .." {
            // Up a directory.
            path.pop();
        } else if line == "$ cd /" {
            // Back to top, this doesn't occur other than at the top of the
            // input, so this part could've been skipped.
            path.clear();
        } else if let Ok((size,)) = file_entry(&line) {
            // Add file size to current path and to all paths above it.
            let seen_key = (path.clone(), size);
            if !seen.contains(&seen_key) {
                let mut prefixes = path.clone();
                loop {
                    *subdir_sizes.entry(prefixes.clone()).or_insert(0) += size;
                    if prefixes.is_empty() {
                        break;
                    }
                    prefixes.pop();
                }
            }
            seen.insert(seen_key);
        } else if let Ok((subdir,)) = subdir_cd(&line) {
            // Enter subdirectory.
            path.push(subdir.clone());
        }
    }

    let mut sizes: Vec<usize> = subdir_sizes.into_values().collect();
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
