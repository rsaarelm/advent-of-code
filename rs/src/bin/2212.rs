use std::collections::{HashMap, HashSet};

use aoc::prelude::*;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Cell {
    Start,
    End,
    Hill(u32),
}

use Cell::*;

impl From<char> for Cell {
    fn from(c: char) -> Self {
        match c {
            'S' => Start,
            'E' => End,
            c => Hill(c as u32 - 'a' as u32),
        }
    }
}

impl Cell {
    pub fn height(self) -> u32 {
        match self {
            Start => 0,
            End => 'z' as u32 - 'a' as u32,
            Hill(x) => x,
        }
    }
}

fn main() {
    let (w, h, grid) = stdin_grid_into::<Cell>();

    let neighbors = |(x, y): (usize, usize)| {
        let mut ret = Vec::new();
        let a = grid[y][x].height();
        if x > 0 && grid[y][x - 1].height() <= a + 1 {
            ret.push((x - 1, y));
        }
        if y > 0 && grid[y - 1][x].height() <= a + 1 {
            ret.push((x, y - 1));
        }
        if x < w - 1 && grid[y][x + 1].height() <= a + 1 {
            ret.push((x + 1, y));
        }
        if y < h - 1 && grid[y + 1][x].height() <= a + 1 {
            ret.push((x, y + 1));
        }

        ret
    };

    let mut start = (0, 0);
    let mut end = (0, 0);
    for (y, line) in grid.iter().enumerate() {
        for (x, t) in line.iter().enumerate() {
            if *t == Start {
                start = (x, y);
            } else if *t == End {
                end = (x, y);
            }
        }
    }

    // Part 1

    let mut dist_to = HashMap::new();
    let mut edge = HashSet::from([(0, start)]);

    while let Some((len, node)) = edge.pop() {
        dist_to.insert(node, len);

        for n in neighbors(node) {
            if dist_to.get(&n).copied().unwrap_or(u32::MAX) > len + 1 {
                edge.insert((len + 1, n));
            }
        }
    }

    println!("{}", dist_to[&end]);

    // Part 2

    // Moving backwards, different formula.
    let neighbors = |(x, y): (usize, usize)| {
        let mut ret = Vec::new();
        let a = grid[y][x].height();
        if x > 0 && grid[y][x - 1].height() + 1 >= a {
            ret.push((x - 1, y));
        }
        if y > 0 && grid[y - 1][x].height() + 1 >= a {
            ret.push((x, y - 1));
        }
        if x < w - 1 && grid[y][x + 1].height() + 1 >= a {
            ret.push((x + 1, y));
        }
        if y < h - 1 && grid[y + 1][x].height() + 1 >= a {
            ret.push((x, y + 1));
        }

        ret
    };

    let mut dist_to = HashMap::new();
    let mut edge = HashSet::from([(0, end)]);
    let mut starts = Vec::new();

    while let Some((len, node)) = edge.pop() {
        dist_to.insert(node, len);
        if grid[node.1][node.0].height() == 0 {
            starts.push(len);
        }

        for n in neighbors(node) {
            if dist_to.get(&n).copied().unwrap_or(u32::MAX) > len + 1 {
                edge.insert((len + 1, n));
            }
        }
    }

    starts.sort();
    println!("{}", starts[0]);
}
