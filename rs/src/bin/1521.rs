use std::ops::Range;

use aoc::prelude::*;
use nalgebra::Vector4;

trait Entity {
    fn cost(&self) -> i32;
    fn hp(&self) -> i32;
    fn dmg(&self) -> i32;
    fn armor(&self) -> i32;

    fn beats(&self, other: &Self) -> bool {
        let self_dmg = (self.dmg() - other.armor()).max(1);
        let other_dmg = (other.dmg() - self.armor()).max(1);

        let self_lasts = (self.hp() + other_dmg - 1) / other_dmg;
        let other_lasts = (other.hp() + self_dmg - 1) / self_dmg;

        self_lasts >= other_lasts
    }
}

impl Entity for Vector4<i32> {
    fn cost(&self) -> i32 {
        self[0]
    }

    fn hp(&self) -> i32 {
        self[1]
    }

    fn dmg(&self) -> i32 {
        self[2]
    }

    fn armor(&self) -> i32 {
        self[3]
    }
}

const ITEMS: [Vector4<i32>; 17] = [
    // Additive identity
    Vector4::new(0, 0, 0, 0),
    // 1..5: weapons
    Vector4::new(8, 0, 4, 0),
    Vector4::new(10, 0, 5, 0),
    Vector4::new(25, 0, 6, 0),
    Vector4::new(40, 0, 7, 0),
    Vector4::new(74, 0, 8, 0),
    // 6..10: armors
    Vector4::new(13, 0, 0, 1),
    Vector4::new(31, 0, 0, 2),
    Vector4::new(53, 0, 0, 3),
    Vector4::new(75, 0, 0, 4),
    Vector4::new(102, 0, 0, 5),
    // 11..16: rings
    Vector4::new(25, 0, 1, 0),
    Vector4::new(50, 0, 2, 0),
    Vector4::new(100, 0, 3, 0),
    Vector4::new(20, 0, 0, 1),
    Vector4::new(40, 0, 0, 2),
    Vector4::new(80, 0, 0, 3),
];

const WEAPONS: Range<usize> = 1..6;
const ARMORS: Range<usize> = 6..11;
const RINGS: Range<usize> = 11..17;

fn main() {
    let [boss_hp, boss_dmg, boss_armor] =
        fixed_numbers::<i32, 3>(stdin_string());
    let boss = Vector4::new(0, boss_hp, boss_dmg, boss_armor);
    let player = Vector4::new(0, 100, 0, 0);

    let mut winning_loadouts = Vec::new();
    let mut losing_loadouts = Vec::new();
    for w in WEAPONS.clone() {
        for a in ARMORS.clone().chain(Some(0)) {
            for r1 in RINGS.clone().chain(Some(0)) {
                for r2 in RINGS.clone().chain(Some(0)) {
                    if r2 != 0 && r2 == r1 {
                        continue;
                    }

                    let kitted_player =
                        player + ITEMS[w] + ITEMS[a] + ITEMS[r1] + ITEMS[r2];
                    if kitted_player.beats(&boss) {
                        winning_loadouts.push(kitted_player.cost());
                    } else {
                        losing_loadouts.push(kitted_player.cost());
                    }
                }
            }
        }
    }

    winning_loadouts.sort();
    println!("{}", winning_loadouts[0]);
    losing_loadouts.sort_by_key(|e| -e);
    println!("{}", losing_loadouts[0]);
}
