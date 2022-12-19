use std::{collections::BinaryHeap, str::FromStr};

use aoc::prelude::*;
use rustc_hash::FxHashSet;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct State {
    resources: IVec3,
    robots: IVec3,
    time_left: usize,
    geodes: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.max_geodes().cmp(&other.max_geodes())
        //self.geodes.cmp(&other.geodes)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl State {
    fn new(time_left: usize) -> Self {
        State {
            resources: ivec3(0, 0, 0),
            robots: ivec3(1, 0, 0),
            time_left,
            geodes: 0,
        }
    }

    /// How many geodes this plan could build at best.
    fn max_geodes(&self) -> usize {
        if self.time_left == 0 {
            self.geodes
        } else {
            self.geodes + (self.time_left * (self.time_left + 1)) / 2
        }
    }

    fn options(&self, blueprint: &Blueprint) -> Vec<State> {
        if self.time_left == 0 {
            return Vec::new();
        }

        let mut ret = Vec::new();

        // No-op state, just grow resources and shrink time.
        let default_next = State {
            resources: self.resources + self.robots,
            time_left: self.time_left - 1,
            robots: self.robots,
            geodes: self.geodes,
        };

        ret.push(default_next);

        // States for building one of each type of robot we have resources
        // for.
        for (res, cost) in
            [blueprint.ore, blueprint.cla, blueprint.obs, blueprint.geo]
                .into_iter()
                .enumerate()
        {
            if cost.cmpgt(self.resources).any() {
                continue;
            }
            let mut state = default_next;
            state.resources -= cost;
            if res == 3 {
                // Built a geode robot, just add its total future output to
                // score.
                state.geodes += state.time_left;
            } else {
                // Built a resource robot, add to vector.
                state.robots[res] += 1;
            }

            ret.push(state);
        }

        ret
    }
}

#[derive(Default, Copy, Clone, Debug)]
struct Blueprint {
    ore: IVec3,
    cla: IVec3,
    obs: IVec3,
    geo: IVec3,
}

impl FromStr for Blueprint {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // XXX: Will just panic instead of erroring on bad input.
        let [_id, ore_0, ore_1, ore_2, cla_2, ore_3, obs_3] =
            fixed_numbers::<i32, 7>(s);
        Ok(Blueprint {
            ore: ivec3(ore_0, 0, 0),
            cla: ivec3(ore_1, 0, 0),
            obs: ivec3(ore_2, cla_2, 0),
            geo: ivec3(ore_3, 0, obs_3),
        })
    }
}

fn geodes_opened(time: usize, blueprint: &Blueprint) -> usize {
    let state = State::new(time);
    let mut search = BinaryHeap::from([state]);
    let mut seen = FxHashSet::default();

    let mut best_geodes = 0;
    while let Some(next) = search.pop() {
        best_geodes = best_geodes.max(next.geodes);

        for e in next.options(blueprint) {
            // Check max_geodes to see that you're adding a plan that has hope
            // of beating current high score.
            if e.max_geodes() > best_geodes && !seen.contains(&e) {
                seen.insert(e);
                search.push(e);
            }
        }
    }

    best_geodes
}

fn main() {
    let mut blueprints: Vec<Blueprint> =
        stdin_lines().map(|s| s.parse().unwrap()).collect();

    // Part 1

    println!(
        "{}",
        blueprints
            .iter()
            .enumerate()
            .map(|(i, b)| (i + 1) * geodes_opened(24, b))
            .sum::<usize>()
    );

    // Part 2
    blueprints.truncate(3);
    println!(
        "{}",
        blueprints
            .iter()
            .map(|b| geodes_opened(32, b))
            .product::<usize>()
    );
}
