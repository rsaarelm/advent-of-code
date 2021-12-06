use aoc::prelude::*;
use std::collections::{HashSet, VecDeque};
use std::str::FromStr;

#[derive(PartialEq, Eq, Clone, Debug)]
struct GameState {
    decks: [VecDeque<u8>; 2],
    mode: Mode,
    seen: HashSet<[VecDeque<u8>; 2]>,
}

impl FromStr for GameState {
    type Err = ();

    fn from_str(input: &str) -> Result<GameState, Self::Err> {
        let mut decks = [VecDeque::new(), VecDeque::new()];
        let mut i = 0;
        for line in input.lines() {
            if let Ok(num) = line.parse() {
                decks[i].push_back(num);
            }
            if line == "" {
                i = 1;
            }
        }

        Ok(GameState {
            decks,
            mode: Mode::REGULAR,
            seen: HashSet::new(),
        })
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug)]
enum Mode {
    REGULAR,
    RECURSIVE,
}

impl GameState {
    fn recursive(mut self) -> Self {
        self.mode = Mode::RECURSIVE;
        self
    }

    fn score_deck(deck: &VecDeque<u8>) -> u32 {
        deck.iter()
            .rev()
            .enumerate()
            .map(|(i, &c)| c as u32 * (i as u32 + 1))
            .sum()
    }

    fn subdeck(&self, drawn_0: u8, drawn_1: u8) -> GameState {
        let mut ret = GameState {
            decks: self.decks.clone(),
            mode: self.mode,
            seen: HashSet::new(),
        };
        ret.decks[0].truncate(drawn_0 as usize);
        ret.decks[1].truncate(drawn_1 as usize);
        ret
    }

    fn round_winner(&mut self, drawn_0: u8, drawn_1: u8) -> usize {
        if self.can_recurse(drawn_0, drawn_1) {
            self.subdeck(drawn_0, drawn_1).play_to_end()
        } else {
            if drawn_0 > drawn_1 {
                0
            } else {
                1
            }
        }
    }

    fn can_recurse(&self, drawn_0: u8, drawn_1: u8) -> bool {
        self.mode == Mode::RECURSIVE
            && drawn_0 as usize <= self.decks[0].len()
            && drawn_1 as usize <= self.decks[1].len()
    }

    fn pop(&mut self) -> (u8, u8) {
        (
            self.decks[0].pop_front().unwrap(),
            self.decks[1].pop_front().unwrap(),
        )
    }

    fn game_winner(&self) -> Option<usize> {
        if self.decks[0].is_empty() {
            Some(1)
        } else if self.decks[1].is_empty() {
            Some(0)
        } else {
            None
        }
    }

    fn play_to_end(&mut self) -> usize {
        while self.game_winner().is_none() {
            if self.seen.contains(&self.decks) {
                // Player 1 wins.
                self.decks[1].clear();
                break;
            }
            self.seen.insert(self.decks.clone());

            let (drawn_0, drawn_1) = self.pop();
            match self.round_winner(drawn_0, drawn_1) {
                0 => {
                    self.decks[0].push_back(drawn_0);
                    self.decks[0].push_back(drawn_1);
                }
                1 => {
                    self.decks[1].push_back(drawn_1);
                    self.decks[1].push_back(drawn_0);
                }
                _ => panic!("Invalid winner"),
            }
        }
        self.game_winner().unwrap()
    }

    pub fn play(mut self) -> u32 {
        GameState::score_deck(&self.decks[self.play_to_end()])
    }
}

fn run_1(input: &str) -> u32 {
    let state = input.parse::<GameState>().unwrap();
    state.play()
}

fn run_2(input: &str) -> u32 {
    let state = input.parse::<GameState>().unwrap().recursive();
    state.play()
}

fn main() {
    let input = stdin_string();
    println!("{}", run_1(&input));
    println!("{}", run_2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            306,
            run_1(
                "\
Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10"
            )
        );
    }

    #[test]
    fn test_loop() {
        assert_eq!(
            105,
            run_2(
                "\
Player 1:
43
19

Player 2:
2
29
14"
            )
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            291,
            run_2(
                "\
Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10"
            )
        );
    }
}
