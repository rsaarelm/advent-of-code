use aoc::prelude::*;
use glam::IVec3;
use itertools::Itertools;
use rayon::prelude::*;
use std::{
    convert::TryInto,
    f32,
    ops::{Deref, DerefMut},
    sync::LazyLock,
};

/// Transform a vector into 24 possible axis-aligned cube orientations.
fn rotations(vec: impl Into<IVec3>) -> [IVec3; 24] {
    use glam::{EulerRot, Mat3A};

    static ORIENTATIONS: LazyLock<Vec<Mat3A>> = LazyLock::new(|| {
        let mut ret = Vec::new();
        for &(xa, ya) in &[
            // Facings:
            // front,
            (0.0, 0.0),
            // sides
            (0.0, f32::consts::FRAC_PI_2),
            (0.0, -f32::consts::FRAC_PI_2),
            // back,
            (0.0, f32::consts::PI),
            // up and down.
            (f32::consts::FRAC_PI_2, 0.0),
            (-f32::consts::FRAC_PI_2, 0.0),
        ] {
            // Different up vectors.
            for &za in &[
                0.0,
                f32::consts::FRAC_PI_2,
                f32::consts::PI,
                -f32::consts::FRAC_PI_2,
            ] {
                ret.push(Mat3A::from_euler(EulerRot::ZXY, za, xa, ya));
            }
        }
        ret
    });

    let vec = vec.into().as_vec3a();

    ORIENTATIONS
        .iter()
        .map(|&m| (m * vec).round().as_ivec3())
        .collect::<Vec<IVec3>>()
        .as_slice()
        .try_into()
        .unwrap()
}

#[derive(Clone, Default, Debug)]
struct Region(Vec<IVec3>);

impl Deref for Region {
    type Target = Vec<IVec3>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Region {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Region {
    /// Return list of vectors which when added to points of current region
    /// will align them with at least one point of the other region.
    pub fn match_candidates(&self, other: &Region) -> HashSet<IVec3> {
        self.iter()
            .cartesian_product(other.iter())
            .map(|(&a, &b)| b - a)
            .collect()
    }

    pub fn variants(&self) -> [Region; 24] {
        let mut ret: [Region; 24] = Default::default();
        for &p in self.iter() {
            for (i, p) in IntoIterator::into_iter(rotations(p)).enumerate() {
                ret[i].push(p);
            }
        }
        ret
    }

    /// Return true if the translation matches this with the other region.
    pub fn matches(&self, translation: IVec3, other: &Region) -> bool {
        const EXPECTED_MATCHES: usize = 12;

        // XXX: This doesn't check if this Scanner knows beacons that are in
        // the other's scan region but not accounted by the other.
        self.iter()
            .map(|&p| p + translation)
            .cartesian_product(other.iter())
            .filter(|(u, v)| u == *v)
            .count()
            >= EXPECTED_MATCHES
    }

    /// Expand self with translated points of other region.
    pub fn expand(&mut self, translation: IVec3, other: &Region) {
        for &p in other.iter() {
            let p = p + translation;
            if !self.contains(&p) {
                self.push(p);
            }
        }
    }
}

fn main() {
    let regions: Vec<Region> = stdin_string()
        .trim()
        .split("\n\n")
        .map(|chunk| {
            Region(
                chunk
                    .lines()
                    .skip(1)
                    .map(fixed_numbers)
                    .map(|v: [i32; 3]| IVec3::from(v))
                    .collect::<Vec<IVec3>>(),
            )
        })
        .collect();

    let mut space = regions[0].clone();
    // Unmatched regions.
    let mut open: Vec<[Region; 24]> =
        regions.iter().skip(1).map(|r| r.variants()).collect();

    // Centers of matched regions. The first one is origin by definition.
    let mut posns = vec![IVec3::new(0, 0, 0)];

    while !open.is_empty() {
        if let Some((i, j, pos)) = (0..open.len())
            .cartesian_product(0..24)
            .collect::<Vec<(usize, usize)>>()
            .par_iter()
            .find_map_first(|(i, j)| {
                let region = &open[*i][*j];
                region
                    .match_candidates(&space)
                    .par_iter()
                    .find_any(|&&p| region.matches(p, &space))
                    .map(|&p| (*i, *j, p))
            })
        {
            eprintln!("Index {} of {} matched with {:?}.", i, open.len(), pos);
            space.expand(pos, &open[i][j]);
            open.swap_remove(i);
            posns.push(pos);
        } else {
            panic!("Did not find a matching region.");
        }
    }

    println!("{}", space.len());

    // 2
    let max_dist = posns
        .iter()
        .cartesian_product(posns.iter())
        .map(|(&a, &b)| (a - b).dot([1, 1, 1].into()))
        .max()
        .unwrap_or(0);
    println!("{}", max_dist);
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustc_hash::FxHashSet as HashSet;

    #[test]
    fn test_rotations() {
        let uniqs = |v| {
            IntoIterator::into_iter(rotations(v))
                .collect::<HashSet<_>>()
                .len()
        };

        assert_eq!(uniqs([0, 0, 0]), 1);

        assert_eq!(uniqs([1, 0, 0]), 6);
        assert_eq!(uniqs([0, 1, 0]), 6);
        assert_eq!(uniqs([0, 0, 1]), 6);

        assert_eq!(uniqs([1, 2, 3]), 24);
    }
}
