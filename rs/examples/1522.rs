use aoc::prelude::*;

#[derive(Clone, Eq, PartialEq, Hash)]
struct World {
    boss_hp: i32,
    boss_dmg: i32,
    hard_mode: bool,
    hp: i32,
    mana: i32,
    spent_mana: i32,
    shield: usize,
    poison: usize,
    recharge: usize,
}

#[derive(Copy, Clone)]
enum Spell {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

use Spell::*;

impl World {
    fn new(boss_hp: i32, boss_dmg: i32) -> Self {
        World {
            boss_hp,
            boss_dmg,
            hard_mode: false,
            hp: 50,
            mana: 500,
            spent_mana: 0,
            shield: 0,
            poison: 0,
            recharge: 0,
        }
    }

    fn hard_mode(mut self) -> Self {
        self.hard_mode = true;
        self
    }

    fn step_effects(&mut self) {
        if self.poison > 0 {
            self.boss_hp -= 3;
            self.poison -= 1;
        }
        if self.shield > 0 {
            self.shield -= 1;
        }
        if self.recharge > 0 {
            self.mana += 101;
            self.recharge -= 1;
        }
    }

    fn enemy_turn(&mut self) {
        if self.boss_hp > 0 {
            let dmg = if self.shield > 0 {
                self.boss_dmg - 7
            } else {
                self.boss_dmg
            };

            self.hp -= dmg.max(1);
        }
    }

    fn won(&self) -> bool {
        self.hp > 0 && self.boss_hp <= 0
    }

    fn consume_mana(&mut self, amount: i32) {
        self.mana -= amount;
        self.spent_mana += amount;

        // Running out of mana kills you.
        if self.mana < 0 {
            self.hp = 0;
        }
    }

    fn cast(&mut self, spell: Spell) {
        if self.hard_mode {
            self.hp -= 1;
        }
        if self.hp <= 0 {
            return;
        }

        self.step_effects();

        match spell {
            MagicMissile => {
                self.consume_mana(53);
                self.boss_hp -= 4;
            }
            Drain => {
                self.consume_mana(73);
                if self.hp > 0 {
                    self.hp += 2;
                }
                self.boss_hp -= 2;
            }
            Shield => {
                self.consume_mana(113);
                if self.shield != 0 {
                    // Trying to cast an effect twice kills you.
                    self.hp = 0;
                }
                self.shield = 6;
            }
            Poison => {
                self.consume_mana(173);
                if self.poison != 0 {
                    self.hp = 0;
                }
                self.poison = 6;
            }
            Recharge => {
                self.consume_mana(229);
                if self.recharge != 0 {
                    self.hp = 0;
                }
                self.recharge = 5;
            }
        }

        self.step_effects();
        self.enemy_turn();
    }

    fn neighbors(&self) -> Vec<World> {
        [MagicMissile, Drain, Shield, Poison, Recharge]
            .into_iter()
            .filter_map(|spell| {
                let mut w = self.clone();
                w.cast(spell);
                (w.hp > 0).then_some(w)
            })
            .collect()
    }

    fn heuristic(&self) -> i32 {
        if self.won() {
            0
        } else {
            self.spent_mana + 1
        }
    }
}

fn main() {
    let [boss_hp, boss_dmg] = fixed_numbers::<i32, 2>(stdin_string());

    let path = astar_search(
        |w| w.neighbors().into_iter(),
        World::heuristic,
        World::new(boss_hp, boss_dmg),
    )
    .unwrap();
    println!("{}", path[0].spent_mana);

    let path = astar_search(
        |w| w.neighbors().into_iter(),
        World::heuristic,
        World::new(boss_hp, boss_dmg).hard_mode(),
    )
    .unwrap();
    println!("{}", path[0].spent_mana);
}
