use aoc::prelude::*;

fn agree(a: &IVec3, b: &IVec3) -> bool {
    for i in 0..3 {
        if a[i] != 0 && b[i] != 0 && a[i].signum() != b[i].signum() {
            return false;
        }
    }
    true
}

struct System(Vec<(IVec3, IVec3, IVec3)>);

impl System {
    /// Return true if system is still unstable.
    fn update(&mut self) -> bool {
        let mut unstable = false;
        for (p, v, a) in self.0.iter_mut() {
            *v += *a;
            *p += *v;

            if !agree(p, v) || !agree(v, a) {
                unstable = true;
            }
        }

        unstable
    }

    fn clean(&mut self) {
        let mut space: HashMap<IVec3, Vec<usize>> = HashMap::default();
        for (i, (p, _, _)) in self.0.iter().enumerate() {
            space.entry(*p).or_default().push(i);
        }
        let mut kill_list: Vec<usize> = space
            .into_values()
            .filter(|a| a.len() > 1)
            .flatten()
            .collect();
        kill_list.sort();

        for &i in kill_list.iter().rev() {
            self.0.swap_remove(i);
        }
    }

    fn closest_idx(&self) -> usize {
        self.0
            .iter()
            .enumerate()
            .min_by_key(|(_, (p, _, _))| p.x.abs() + p.y.abs() + p.z.abs())
            .unwrap()
            .0
    }
}

fn main() {
    let input: Vec<(IVec3, IVec3, IVec3)> = stdin_lines()
        .map(|line| {
            let [px, py, pz, vx, vy, vz, ax, ay, az]: [i32; 9] =
                fixed_numbers(line);
            (ivec3(px, py, pz), ivec3(vx, vy, vz), ivec3(ax, ay, az))
        })
        .collect();

    let mut system = System(input.clone());
    // Keep evolving the system until every particle's position,
    // velocity and acceleration agree on all signs (zero agrees with
    // anything).
    while system.update() {}
    println!("{}", system.closest_idx());

    // P2
    let mut system = System(input.clone());
    while system.update() {
        system.clean();
    }
    println!("{}", system.0.len());
}
