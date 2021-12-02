//! Day 1:
//! This solution uses a sliding window of 2 values for the first part. In the
//! second part we expand the window to 4 values as
//! `[a, b, c].sum() - [b, c, d].sum() == a - d`, so we only need to consider
//! the first and last values to determine if the three value windows increase
//! or decrease.

use crate::prelude::*;

// -----------------------------------------------------------------------------
// Part 1
// -----------------------------------------------------------------------------
fn part_1(depths: &Vec<i32>) -> crate::Result<i32> {
    Ok(depths.windows(2).filter(|pair| pair[1] > pair[0]).count() as i32)
}

// -----------------------------------------------------------------------------
// Part 2
// -----------------------------------------------------------------------------
fn part_2(depths: &Vec<i32>) -> crate::Result<i32> {
    Ok(depths
        .windows(4)
        .filter(|quartet| quartet[3] > quartet[0])
        .count() as i32)
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
    let depths: Vec<i32> = buffer
        .lines()
        .map(|line| line.parse().expect("failed to parse line"))
        .collect();
    let time_setup = start_setup.elapsed();

    // -------------------------------------------------------------------------
    // Part 1
    // -------------------------------------------------------------------------
    // Look for increases
    let start_part_1 = Instant::now();
    let increases_1 = part_1(&depths)?;
    let time_part_1 = start_part_1.elapsed();

    // -------------------------------------------------------------------------
    // Part 2
    // -------------------------------------------------------------------------
    // Look for increases in window of 3 values
    let start_part_2 = Instant::now();
    let increases_2 = part_2(&depths)?;
    let time_part_2 = start_part_2.elapsed();

    // -------------------------------------------------------------------------
    // Return
    // -------------------------------------------------------------------------
    Ok(RunData::new(
        increases_1 as i64,
        increases_2 as i64,
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
    output::print_day(1, "Sonar Sweep")?;
    output::print_part(1, "📉 Increase", &format!("{}", run_data.part_1))?;
    output::print_part(2, "📉 Increase", &format!("{}", run_data.part_2))?;
    output::print_timing(&run_data.times)?;
    Ok(())
}

// -----------------------------------------------------------------------------
