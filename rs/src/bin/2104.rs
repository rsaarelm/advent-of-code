use aoc::prelude::*;

fn score_if_bingo(board: &Vec<Vec<u32>>, moves: &[u32]) -> Option<u32> {
    // Score accumulation per row and column.
    let mut rows = [0, 0, 0, 0, 0];
    let mut cols = [0, 0, 0, 0, 0];
    for (i, &n) in moves.iter().enumerate() {
        // Only bingo on last move.
        let is_last = i == moves.len() - 1;
        for y in 0..5 {
            for x in 0..5 {
                if board[y][x] == n {
                    rows[y] += 1;
                    cols[x] += 1;
                }
                if rows[y] == 5 || cols[x] == 5 {
                    if is_last {
                        let score = moves[moves.len() - 1]
                            * board
                                .iter()
                                .map(|row| row.iter())
                                .flatten()
                                .filter(|&n| moves.iter().position(|i| i == n).is_none())
                                .sum::<u32>();
                        return Some(score);
                    } else {
                        // Stale bingo
                        return None;
                    }
                }
            }
        }
    }

    None
}

fn main() {
    let parts: Vec<String> = stdin_string()
        .trim()
        .split("\n\n")
        .map(|s| s.to_owned())
        .collect();
    let moves: Vec<u32> = numbers(&parts[0]);
    let boards: Vec<Vec<Vec<u32>>> = parts
        .into_iter()
        .skip(1)
        .map(|b| b.split('\n').map(numbers).collect())
        .collect();

    'part1: for i in 1..moves.len() {
        let moves = &moves[0..i];
        for board in &boards {
            if let Some(score) = score_if_bingo(board, moves) {
                println!("{}", score);
                break 'part1;
            }
        }
    }

    'part2: for i in (1..moves.len()).rev() {
        let moves = &moves[0..i];
        for board in &boards {
            if let Some(score) = score_if_bingo(board, moves) {
                println!("{}", score);
                break 'part2;
            }
        }
    }
}
