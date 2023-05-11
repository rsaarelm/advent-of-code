use serde::Deserialize;

use aoc::prelude::*;

#[derive(Copy, Clone, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
enum HexDir {
    N,
    Ne,
    Se,
    S,
    Sw,
    Nw,
}

use HexDir::*;

impl HexDir {
    fn dir(&self) -> IVec2 {
        match self {
            N => ivec2(-1, -1),
            Ne => ivec2(0, -1),
            Se => ivec2(1, 0),
            S => ivec2(1, 1),
            Sw => ivec2(0, 1),
            Nw => ivec2(-1, 0),
        }
    }
}

fn hex_abs(vec: IVec2) -> i32 {
    if vec.x.signum() == vec.y.signum() {
        vec.x.abs().max(vec.y.abs())
    } else {
        vec.x.abs() + vec.y.abs()
    }
}

fn main() {
    let input = stdin_string();
    let dirs: Vec<HexDir> = input
        .split(',')
        .map(|a| idm::from_str(a).unwrap())
        .collect();

    println!(
        "{}",
        hex_abs(dirs.iter().map(|a| a.dir()).reduce(|a, b| a + b).unwrap())
    );

    println!(
        "{}",
        dirs.iter()
            .map(|a| a.dir())
            .fold((0, ivec2(0, 0)), |(m, p), d| (m.max(hex_abs(p + d)), p + d))
            .0
    );
}
