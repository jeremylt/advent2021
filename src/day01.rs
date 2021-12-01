//! Day 1:
//! This solution uses a sliding window of 2 values for the first part. However,
//! for the second part the windows approach is less effecient and we move to a
//! running sum of three value as we iterate through the depth values.

use crate::prelude::*;

// -----------------------------------------------------------------------------
// Part 1
// -----------------------------------------------------------------------------
fn part_1(depths: &Vec<i32>) -> crate::Result<i32> {
    Ok(depths.windows(2).fold(0, |count, pair| {
        count + if pair[1] > pair[0] { 1 } else { 0 }
    }))
}

// -----------------------------------------------------------------------------
// Part 2
// -----------------------------------------------------------------------------
fn part_2(depths: &Vec<i32>) -> crate::Result<i32> {
    let mut sum = depths[0] + depths[1] + depths[2];
    Ok(depths
        .iter()
        .enumerate()
        .skip(3)
        .fold(0, |count, (index, current)| {
            let old_sum = sum;
            sum += current;
            sum -= depths[index - 3];
            count + if sum > old_sum { 1 } else { 0 }
        }))
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
