//! Day 10:
//! I used the typical algorith for brace matches. The slowest part of this
//! approch is the parsing of each line, with the dynamic creation of the vector
//! of opening braces.

use crate::prelude::*;

// -----------------------------------------------------------------------------
// Direction data struct
// -----------------------------------------------------------------------------
#[derive(Default, Debug)]
struct NavigationLine {
    characters: Vec<u8>,
}

impl std::str::FromStr for NavigationLine {
    type Err = crate::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut characters = Vec::with_capacity(20);
        s.as_bytes().iter().find(|character| match **character {
            b'(' | b'[' | b'{' | b'<' => {
                characters.push(**character);
                false
            }
            b')' | b']' | b'}' | b'>' => {
                let last_opening = characters.pop().unwrap_or(u8::MAX);
                if last_opening > **character || **character - last_opening > 2 {
                    characters = vec![**character];
                    true
                } else {
                    false
                }
            }
            _ => unreachable!(),
        });
        Ok(Self { characters })
    }
}

impl NavigationLine {
    fn score_mismatch(&self) -> u32 {
        match self.characters[0] {
            b')' => 3,
            b']' => 57,
            b'}' => 1197,
            b'>' => 25137,
            _ => 0,
        }
    }

    fn score_missing(&self) -> u64 {
        self.characters.iter().rev().fold(0, |score, current| {
            score * 5
                + match current {
                    b'(' => 1,
                    b'[' => 2,
                    b'{' => 3,
                    b'<' => 4,
                    _ => 0,
                }
        })
    }
}

// -----------------------------------------------------------------------------
// Part 1
// -----------------------------------------------------------------------------
fn part_1(chunks: &Vec<NavigationLine>) -> crate::Result<u32> {
    Ok(chunks.iter().map(|chunk| chunk.score_mismatch()).sum())
}

// -----------------------------------------------------------------------------
// Part 2
// -----------------------------------------------------------------------------
fn part_2(chunks: &Vec<NavigationLine>) -> crate::Result<u64> {
    let mut scores: Vec<u64> = chunks
        .iter()
        .map(|chunk| chunk.score_missing())
        .filter(|score| *score != 0)
        .collect();
    scores.sort_unstable();
    Ok(scores[scores.len() / 2])
}

// -----------------------------------------------------------------------------
// Combined
// -----------------------------------------------------------------------------
fn combined(chunks: &mut dyn Iterator<Item = NavigationLine>) -> crate::Result<(u32, u64)> {
    let mut score_mismatch = 0;
    let mut scores_missing = Vec::with_capacity(20);
    chunks.for_each(|chunk| {
        if chunk.characters.len() == 1 {
            score_mismatch += chunk.score_mismatch();
        } else {
            scores_missing.push(chunk.score_missing());
        }
    });
    scores_missing.sort_unstable();
    let score_missing = scores_missing[scores_missing.len() / 2];
    Ok((score_mismatch, score_missing))
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
    let chunks: Vec<NavigationLine> = buffer
        .lines()
        .map(|line| line.parse().expect("failed to parse line"))
        .collect();
    let time_setup = start_setup.elapsed();

    // -------------------------------------------------------------------------
    // Part 1
    // -------------------------------------------------------------------------
    // Score brace mismatches
    let start_part_1 = Instant::now();
    let score_1 = part_1(&chunks)?;
    let time_part_1 = start_part_1.elapsed();

    // -------------------------------------------------------------------------
    // Part 2
    // -------------------------------------------------------------------------
    // Score incomplete lines
    let start_part_2 = Instant::now();
    let score_2 = part_2(&chunks)?;
    let time_part_2 = start_part_2.elapsed();

    // -------------------------------------------------------------------------
    // Combined
    // -------------------------------------------------------------------------
    let start_combined = Instant::now();
    let mut chunks = buffer
        .lines()
        .map(|line| line.parse().expect("failed to parse line"));
    let (score_1_combined, score_2_combined) = combined(&mut chunks)?;
    let time_combined = start_combined.elapsed();
    assert_eq!(score_1, score_1_combined);
    assert_eq!(score_2, score_2_combined);

    // -------------------------------------------------------------------------
    // Return
    // -------------------------------------------------------------------------
    Ok(RunData::new(
        score_1 as i64,
        score_2 as i64,
        Timing::new(time_setup, time_part_1, time_part_2, time_combined),
    ))
}

// -----------------------------------------------------------------------------
// Report
// -----------------------------------------------------------------------------
pub(crate) fn report(run_data: &RunData) -> crate::Result<()> {
    output::print_day(10, "Syntax Scoring")?;
    output::print_part(1, "📉 Score", &format!("{}", run_data.part_1))?;
    output::print_part(2, "📉 Score", &format!("{}", run_data.part_2))?;
    output::print_timing(&run_data.times)?;
    Ok(())
}

// -----------------------------------------------------------------------------
