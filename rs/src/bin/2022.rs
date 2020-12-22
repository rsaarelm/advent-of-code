use aoc::util;
use std::collections::{HashSet, VecDeque};
use std::str::FromStr;

#[derive(PartialEq, Eq, Clone, Hash, Debug)]
struct GameState {
    decks: [VecDeque<usize>; 2],
    mode: Mode,
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

    fn score_deck(deck: &VecDeque<usize>) -> usize {
        deck.iter()
            .rev()
            .enumerate()
            .map(|(i, c)| c * (i + 1))
            .sum()
    }

    fn round_winner(&self, seen: &mut HashSet<GameState>, drawn_0: usize, drawn_1: usize) -> usize {
        if seen.contains(self) {
            return 0;
        }
        seen.insert(self.clone());

        if self.can_recurse(drawn_0, drawn_1) {
            self.clone().play_to_end(seen)
        } else {
            if drawn_0 > drawn_1 {
                0
            } else {
                1
            }
        }
    }

    fn can_recurse(&self, drawn_0: usize, drawn_1: usize) -> bool {
        self.mode == Mode::RECURSIVE
            && drawn_0 <= self.decks[0].len()
            && drawn_1 <= self.decks[1].len()
    }

    fn pop(&mut self) -> (usize, usize) {
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

    fn play_to_end(&mut self, seen: &mut HashSet<GameState>) -> usize {
        while self.game_winner().is_none() {
            let (drawn_0, drawn_1) = self.pop();
            match self.round_winner(seen, drawn_0, drawn_1) {
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

    pub fn play(mut self) -> usize {
        let mut seen = HashSet::new();

        let winner = self.play_to_end(&mut seen);
        GameState::score_deck(&self.decks[winner])
    }
}

fn run_1(input: &str) -> usize {
    let state = input.parse::<GameState>().unwrap();
    state.play()
}

fn run_2(input: &str) -> usize {
    let state = input.parse::<GameState>().unwrap().recursive();
    state.play()
}

fn main() {
    let input = util::slurp_stdin();
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
            273,
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
