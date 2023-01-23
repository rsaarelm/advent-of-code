use std::collections::BTreeSet;

use lazy_static::lazy_static;
use rustc_hash::FxHashMap as HashMap;

pub type PointCloud = BTreeSet<[i32; 2]>;

lazy_static! {
    static ref LETTERS: HashMap<PointCloud, char> = {
        const GLYPHS: &str = "\
.##.
#..#
#..#
####
#..#
#..#

..##..
.#..#.
#....#
#....#
#....#
######
#....#
#....#
#....#
#....#

###.
#..#
###.
#..#
#..#
###.

#####.
#....#
#....#
#....#
#####.
#....#
#....#
#....#
#....#
#####.

.##.
#..#
#...
#...
#..#
.##.

.####.
#....#
#.....
#.....
#.....
#.....
#.....
#.....
#....#
.####.

####
#...
###.
#...
#...
####

######
#.....
#.....
#.....
#####.
#.....
#.....
#.....
#.....
######

####
#...
###.
#...
#...
#...

######
#.....
#.....
#.....
#####.
#.....
#.....
#.....
#.....
#.....

.##.
#..#
#...
#.##
#..#
.###

.####.
#....#
#.....
#.....
#.....
#..###
#....#
#....#
#...##
.###.#

#..#
#..#
####
#..#
#..#
#..#

#....#
#....#
#....#
#....#
######
#....#
#....#
#....#
#....#
#....#

###
.#.
.#.
.#.
.#.
###

..##
...#
...#
...#
#..#
.##.

...###
....#.
....#.
....#.
....#.
....#.
....#.
#...#.
#...#.
.###..

#..#
#.#.
##..
#.#.
#.#.
#..#

#....#
#...#.
#..#..
#.#...
##....
##....
#.#...
#..#..
#...#.
#....#

#...
#...
#...
#...
#...
####

#.....
#.....
#.....
#.....
#.....
#.....
#.....
#.....
#.....
######

#....#
##...#
##...#
#.#..#
#.#..#
#..#.#
#..#.#
#...##
#...##
#....#

.##.
#..#
#..#
#..#
#..#
.##.

###.
#..#
#..#
###.
#...
#...

#####.
#....#
#....#
#....#
#####.
#.....
#.....
#.....
#.....
#.....

###.
#..#
#..#
###.
#.#.
#..#

#####.
#....#
#....#
#....#
#####.
#..#..
#...#.
#...#.
#....#
#....#

.###
#...
#...
.##.
...#
###.

#..#
#..#
#..#
#..#
#..#
.##.

#....#
#....#
.#..#.
.#..#.
..##..
..##..
.#..#.
.#..#.
#....#
#....#

####
...#
..#.
.#..
#...
####

######
.....#
.....#
....#.
...#..
..#...
.#....
#.....
#.....
######";

        const CHARS: &str = "AABBCCEEFFGGHHIJJKKLLNOPPRRSUXZZ";
        debug_assert_eq!(GLYPHS.split("\n\n").count(), CHARS.len());
        GLYPHS
            .split("\n\n")
            .map(points)
            .zip(CHARS.chars())
            .collect()
    };
}

struct Shape {
    offset: (i32, i32),
    points: PointCloud,
}

impl Shape {
    /// Extract one connected shape from point cloud and remove the shape's
    /// points from the cloud. Returns `None` if cloud is empty.
    pub fn extract(cloud: &mut PointCloud) -> Option<Self> {
        let Some(&seed) = cloud.iter().next() else {
            return None;
        };
        let mut edge = vec![seed];
        let mut points = PointCloud::new();

        let mut min_x = seed[0];
        let mut min_y = seed[1];

        while let Some([x, y]) = edge.pop() {
            points.insert([x, y]);
            cloud.remove(&[x, y]);

            min_x = min_x.min(x);
            min_y = min_y.min(y);

            for v in -1..=1 {
                for u in -1..=1 {
                    if u == 0 && v == 0 {
                        continue;
                    }

                    let p = [x + u, y + v];
                    if cloud.contains(&p) && !points.contains(&p) {
                        edge.push(p);
                    }
                }
            }
        }

        let offset = (min_x, min_y);
        points = points
            .into_iter()
            .map(|[x, y]| [x - min_x, y - min_y])
            .collect();

        Some(Shape { offset, points })
    }
}

pub fn ocr<'a>(
    input: impl IntoIterator<Item = &'a [i32; 2]>,
) -> Option<String> {
    let mut cloud: PointCloud = input.into_iter().copied().collect();

    let mut shapes: Vec<Shape> =
        std::iter::from_fn(|| Shape::extract(&mut cloud)).collect();
    // XXX: Only works if there is just one line of text.
    shapes.sort_by_key(|a| a.offset.0);

    let mut ret = String::new();
    for shape in &shapes {
        if let Some(c) = LETTERS.get(&shape.points) {
            ret.push(*c);
        } else {
            return None;
        }
    }

    Some(ret)
}

pub fn points<T, I>(input: &str) -> T
where
    T: FromIterator<I>,
    I: From<[i32; 2]>,
{
    // NB. Does not check bounding box, make sure to snap input to x and y
    // axes.
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| {
                if c != '.' && !c.is_whitespace() {
                    Some(I::from([x as i32, y as i32]))
                } else {
                    None
                }
            })
        })
        .collect()
}
