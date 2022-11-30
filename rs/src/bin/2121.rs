use aoc::prelude::*;
use std::collections::HashMap;

#[derive(Debug)]
struct Player {
    slot: i64,
    score: i64,
}

impl Player {
    pub fn new(slot: i64) -> Player {
        let score = 0;
        Player { slot, score }
    }

    pub fn step(&mut self, rolls: &[i64]) {
        for roll in rolls {
            self.slot += roll;
        }
        self.slot = ((self.slot - 1) % 10) + 1;
        self.score += self.slot;
    }

    pub fn has_won(&self) -> bool {
        self.score >= 1000
    }
}

fn play_1(player: &mut [Player; 2]) -> (i64, usize) {
    let die = &mut (1..=100).cycle();
    for round in 1.. {
        let player = &mut player[((round - 1) % 2) as usize];
        player.step(die.take(3).collect::<Vec<i64>>().as_slice());
        if player.has_won() {
            return (round * 3, (round % 2) as usize); // round, loser index
        }
    }
    unreachable!();
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct State {
    generation: i64,
    // pos, score.
    p: [(i64, i64); 2],
}

impl State {
    pub fn new(p1: i64, p2: i64) -> State {
        State {
            generation: 0,
            p: [(p1, 0), (p2, 0)],
        }
    }

    pub fn winner(self) -> Option<usize> {
        for i in 0..2 {
            if self.p[i].1 >= 21 {
                return Some(i);
            }
        }
        None
    }

    pub fn update(self) -> Vec<(State, u64)> {
        let mut ret = Vec::new();

        // Current player index.
        let idx = (self.generation % 2) as usize;

        // Quantum 3d3 can roll 27 ways for 7 different outcomes.
        // Create measure-weighted outcome vector.
        for (&n, d) in [1, 3, 6, 7, 6, 3, 1].iter().zip(3..=9) {
            let mut new_state = self;
            new_state.generation += 1;
            new_state.p[idx].0 = (new_state.p[idx].0 + d - 1) % 10 + 1;
            new_state.p[idx].1 += new_state.p[idx].0;

            ret.push((new_state, n));
        }
        ret
    }
}

fn main() {
    let input = stdin_lines().map(fixed_numbers).collect::<Vec<[i64; 2]>>();

    let [[_, player_1], [_, player_2]] = input.as_slice()
    else { panic!() };

    // 1
    let mut player = [Player::new(*player_1), Player::new(*player_2)];
    let (n_rolls, loser_idx) = play_1(&mut player);
    println!("{}", n_rolls * player[loser_idx].score);

    // 2

    // Game states and their universe counts.
    let mut games_in_progress = HashMap::new();
    games_in_progress.insert(State::new(*player_1, *player_2), 1);

    // States of won games with universe counts.
    let mut won_games = HashMap::new();

    // Propagate the wave function until all branches are finished.
    while !games_in_progress.is_empty() {
        let mut state = HashMap::new();
        for (s, m) in games_in_progress.into_iter() {
            if s.winner().is_some() {
                // Move won game aside.
                *won_games.entry(s).or_insert(0) += m;
            } else {
                // Multiply outcome measure with existing measure.
                for (s, m2) in s.update().into_iter() {
                    *state.entry(s).or_insert(0) += m2 * m;
                }
            }
        }
        games_in_progress = state;
    }

    // Count winning universes.
    let (wins_1, wins_2) = won_games
        .into_iter()
        .map(|(s, n)| {
            if s.winner().unwrap() == 0 {
                (n, 0)
            } else {
                (0, n)
            }
        })
        .fold((0, 0), |(a, b), (i, j)| (a + i, b + j));

    println!("{}", wins_1.max(wins_2));
}
