//! Day 4:

use crate::prelude::*;
use itertools::Itertools;

const NUMBER_ENTRIES: usize = 100;
const NUMBER_SQ_ROOT: usize = 5;

// -----------------------------------------------------------------------------
// Direction data struct
// -----------------------------------------------------------------------------
#[derive(Copy, Clone)]
struct Board {
    entry_rows: [usize; NUMBER_ENTRIES],
    entry_columns: [usize; NUMBER_ENTRIES],
    entry_diagonals: [usize; NUMBER_ENTRIES],
    visited: [bool; NUMBER_ENTRIES],
    // count at index 0 is a dummy value to make behavior same for all balls
    visited_rows: [usize; NUMBER_SQ_ROOT + 1],
    visited_columns: [usize; NUMBER_SQ_ROOT + 1],
    visited_diagonals: [usize; 3],
}

impl Board {
    fn new(board: itertools::Chunk<std::iter::Skip<std::str::Split<&str>>>) -> crate::Result<Self> {
        let mut entry_rows = [0; NUMBER_ENTRIES];
        let mut entry_columns = [0; NUMBER_ENTRIES];
        let mut entry_diagonals = [0; NUMBER_ENTRIES];
        board.into_iter().enumerate().for_each(|(i, line)| {
            line.split_whitespace()
                .into_iter()
                .enumerate()
                .for_each(|(j, number)| {
                    let index: usize = number.trim().parse().expect("failed to parse number");
                    let diagonal = if i == j {
                        1
                    } else if i == NUMBER_SQ_ROOT - j + 1 {
                        2
                    } else {
                        0
                    };
                    entry_rows[index] = i + 1;
                    entry_columns[index] = j + 1;
                    entry_diagonals[index] = diagonal;
                });
        });
        Ok(Self {
            entry_rows: entry_rows,
            entry_columns: entry_columns,
            entry_diagonals: entry_diagonals,
            visited: [false; NUMBER_ENTRIES],
            visited_rows: [0; NUMBER_SQ_ROOT + 1],
            visited_columns: [0; NUMBER_SQ_ROOT + 1],
            visited_diagonals: [0; 3],
        })
    }

    fn is_winner(self) -> crate::Result<bool> {
        Ok(self
            .visited_rows
            .iter()
            .skip(1)
            .any(|visits| *visits == NUMBER_SQ_ROOT)
            || self
                .visited_columns
                .iter()
                .skip(1)
                .any(|visits| *visits == NUMBER_SQ_ROOT)
            || self
                .visited_diagonals
                .iter()
                .skip(1)
                .any(|visits| *visits == NUMBER_SQ_ROOT))
    }

    fn score(self) -> crate::Result<i64> {
        Ok(self
            .visited
            .iter()
            .enumerate()
            .fold(0, |score, (i, visited)| {
                return score
                    + if !*visited && self.entry_rows[i] != 0 {
                        i as i64
                    } else {
                        0
                    };
            }))
    }
}

// -----------------------------------------------------------------------------
// Part 1
// -----------------------------------------------------------------------------
fn part_1(balls: &Vec<usize>, boards: &mut Vec<Board>) -> crate::Result<(i64, usize)> {
    let mut is_winner = false;
    let mut winner = 0;
    let mut winning_ball = 0;
    let mut balls_iter = balls.iter();
    while !is_winner {
        let ball = *balls_iter.next().expect("insufficent balls");
        boards.iter_mut().enumerate().for_each(|(i, board)| {
            board.visited[ball] = true;
            board.visited_rows[board.entry_rows[ball]] += 1;
            board.visited_columns[board.entry_columns[ball]] += 1;
            board.visited_diagonals[board.entry_diagonals[ball]] += 1;
            if !is_winner {
                is_winner = board.is_winner().expect("failed to check status");
                if is_winner {
                    winner = i;
                }
            }
        });
        if is_winner {
            winning_ball = ball;
        }
    }
    let score = boards[winner].score()?;
    Ok((score, winning_ball))
}

// -----------------------------------------------------------------------------
// Part 2
// -----------------------------------------------------------------------------
fn part_2(
    last_ball: usize,
    balls: &Vec<usize>,
    boards: &mut Vec<Board>,
) -> crate::Result<(i64, usize)> {
    let number_boards = boards.len();
    let mut winner_count = 1;
    let mut winner = 0;
    let mut winning_ball = 0;
    let ball_index = balls
        .iter()
        .position(|ball| *ball == last_ball)
        .expect("failed to find ball");
    let mut balls_iter = balls.iter().skip(ball_index + 1);
    while winner_count != number_boards {
        let ball = *balls_iter.next().expect("insufficent balls");
        boards.iter_mut().enumerate().for_each(|(i, board)| {
            let won_before = board.is_winner().expect("failed to check status");
            board.visited[ball] = true;
            board.visited_rows[board.entry_rows[ball]] += 1;
            board.visited_columns[board.entry_columns[ball]] += 1;
            board.visited_diagonals[board.entry_diagonals[ball]] += 1;
            let is_winner = board.is_winner().expect("failed to check status");
            if !won_before && is_winner {
                winner_count += 1;
                if winner_count == number_boards {
                    winner = i;
                }
            }
        });
        if winner_count == number_boards {
            winning_ball = ball;
        }
    }
    let score = boards[winner].score()?;
    Ok((score, winning_ball))
}

// -----------------------------------------------------------------------------
// Run
// -----------------------------------------------------------------------------
pub(crate) fn run(buffer: String) -> crate::Result<RunData> {
    // -------------------------------------------------------------------------
    // Setup
    // -------------------------------------------------------------------------
    // Read to vector
    let start_setup = Instant::now();
    let balls: Vec<usize> = buffer
        .lines()
        .nth(0)
        .expect("failed to parse line")
        .split(",")
        .map(|n| n.parse().expect("failed to parse ball"))
        .collect();
    let mut boards: Vec<Board> = buffer
        .split("\n")
        .skip(2)
        .chunks(6)
        .into_iter()
        .map(|chunk| Board::new(chunk).expect("failed to parse board"))
        .collect();
    let time_setup = start_setup.elapsed();

    // -------------------------------------------------------------------------
    // Part 1
    // -------------------------------------------------------------------------
    // Sum coordinates
    let start_part_1 = Instant::now();
    let (score_1, ball_1) = part_1(&balls, &mut boards)?;
    let product_1 = score_1 * ball_1 as i64;
    let time_part_1 = start_part_1.elapsed();

    // -------------------------------------------------------------------------
    // Part 2
    // -------------------------------------------------------------------------
    // Compute coordinates with aimed directions
    let start_part_2 = Instant::now();
    let (score_2, ball_2) = part_2(ball_1, &balls, &mut boards)?;
    let product_2 = score_2 * ball_2 as i64;
    let time_part_2 = start_part_2.elapsed();

    // -------------------------------------------------------------------------
    // Return
    // -------------------------------------------------------------------------
    Ok(RunData::new(
        product_1 as i64,
        product_2 as i64,
        Timing::new(
            time_setup,
            time_part_1,
            time_part_2,
            std::time::Duration::new(0, 0),
        ),
    ))
}

// -----------------------------------------------------------------------------
// Report
// -----------------------------------------------------------------------------
pub(crate) fn report(run_data: &RunData) -> crate::Result<()> {
    output::print_day(4, "Giant Squid")?;
    output::print_part(1, "📄 Score", &format!("{}", run_data.part_1))?;
    output::print_part(2, "📄 Score", &format!("{}", run_data.part_2))?;
    output::print_timing(&run_data.times)?;
    Ok(())
}

// -----------------------------------------------------------------------------
