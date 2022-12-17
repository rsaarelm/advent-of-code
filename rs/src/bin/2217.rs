use aoc::prelude::*;
use rustc_hash::{FxHashSet, FxHashMap};

// Y-axis is upside down so that the well grows towards positive numbers.

const ROCKS: &str = "\
####

.#.
###
.#.

###
..#
..#

#
#
#
#

##
##";

fn in_well(rock: &FxHashSet<IVec2>) -> bool {
    rock.iter().all(|p| p.x >= 0 && p.x < 7 && p.y >= 0)
}

fn main() {
    let s = stdin_string();
    let winds: Vec<i32> = s
        .chars()
        .map(|c| match c {
            '<' => -1,
            '>' => 1,
            _ => panic!("Bad char"),
        })
        .collect();

    let rocks: Vec<FxHashSet<IVec2>> = ROCKS.split("\n\n").map(points).collect();

    let mut ground = FxHashSet::default();
    let mut top = 0;
    let mut wind_idx = 0;

    // Keep track of what things look like so we can find the point where the
    // cycle repeats.
    let mut deltas: Vec<u64> = Default::default();
    let mut states: FxHashMap<(Vec<i32>, usize, usize), usize> = Default::default();
    let mut loop_start: usize = 0;
    let mut loop_end: usize = 0;

    for n in 0.. {
        let mut rock = rocks[n % rocks.len()]
            .iter()
            .map(|&p| p + ivec2(2, top + 3))
            .collect::<FxHashSet<_>>();

        loop {
            // Push
            let wind = winds[wind_idx % winds.len()];
            wind_idx += 1;

            let rock_2 = rock.iter().map(|&p| p + ivec2(wind, 0)).collect();
            if in_well(&rock_2) && ground.is_disjoint(&rock_2) {
                rock = rock_2;
            }

            // Fall
            let rock_2 = rock.iter().map(|&p| p + ivec2(0, -1)).collect();

            if ground.is_disjoint(&rock_2) && in_well(&rock_2) {
                rock = rock_2;
            } else {
                // Would hit terrain when it falls, merge.
                ground.extend(rock.iter());
                // Stop dropping this rock.
                break;
            }
        }

        // Bookkeeping.

        let new_top = ground.iter().map(|p| p.y).max().unwrap() + 1;
        deltas.push((new_top - top) as u64);
        top = new_top;

        let mut profile: Vec<i32> = (0..7)
            .map(|x| {
                ground
                    .iter()
                    .filter_map(|p| (p.x == x).then_some(p.y))
                    .max()
                    .unwrap_or(0)
            })
            .collect();
        let min = *profile.iter().min().unwrap();
        for x in profile.iter_mut() {
            *x -= min;
        }

        let state_key = (profile, wind_idx % winds.len(), n % rocks.len());
        if states.contains_key(&state_key) {
            // Found the recurrence, can exit now.
            loop_start = states[&state_key];
            loop_end = n;
            break;
        }
        states.insert(state_key, n);
    }

    // Height increase in non-repeating initial section.
    let initial = deltas[0..loop_start].iter().sum::<u64>();
    // Height increase in repeating section.
    let loop_chunk = deltas[loop_start..loop_end].iter().sum::<u64>();
    // Number of rocks in repeating section.
    let loop_size = loop_end - loop_start;

    for x in [2022, 1_000_000_000_000] {
        // FIXME, handle values before the repeating loop.
        assert!(x > loop_start);
        // Skip the non-repeating initial section.
        let x = x - loop_start;

        let mut height = initial + (x / loop_size) as u64 * loop_chunk;
        for i in 0..(x % loop_size) {
            height += deltas[loop_start + i];
        }
        println!("{height}");
    }
}
