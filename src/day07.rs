//! Day 7:
//! The triangle number formula helps tidy up the code here. I am nearly certain
//! that the answer for part 2 has to occur near the average, but I don't have
//! a full proof of this hunch. Sorting is the bottleneck, but using an
//! unstable sort helps somewhat.

use crate::prelude::*;

// -----------------------------------------------------------------------------
// Gauss sum formula for 1 + 2 + 3 + ... + n = n * (n + 1) / 2
// -----------------------------------------------------------------------------
fn gauss_sum(n: u32) -> u32 {
    n * (n + 1) / 2
}

// -----------------------------------------------------------------------------
// Part 1
// -----------------------------------------------------------------------------
fn part_1(positions: &Vec<u16>) -> crate::Result<u32> {
    let median = positions[positions.len() / 2];
    let fuel = positions
        .iter()
        .map(|position| (*position as i32 - median as i32).abs() as u32)
        .sum();
    Ok(fuel)
}

// -----------------------------------------------------------------------------
// Part 2
// -----------------------------------------------------------------------------
fn part_2(positions: &Vec<u16>) -> crate::Result<u32> {
    let average_float =
        positions.iter().map(|n| *n as u32).sum::<u32>() as f32 / positions.len() as f32;
    let average_floor = average_float.floor() as u32;
    let average_ceil = average_float.ceil() as u32;
    let (fuel_floor, fuel_ceil) = positions.iter().fold((0_u32, 0_u32), |acc, position| {
        let distance_floor = (*position as i32 - average_floor as i32).abs() as u32;
        let distance_ceil = (*position as i32 - average_ceil as i32).abs() as u32;
        (
            acc.0 + gauss_sum(distance_floor),
            acc.1 + gauss_sum(distance_ceil),
        )
    });
    Ok(std::cmp::min(fuel_floor, fuel_ceil))
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
    let mut positions: Vec<u16> = buffer
        .trim()
        .split(',')
        .map(|position| position.parse().expect("failed to parse position"))
        .collect();
    positions.sort_unstable();
    let time_setup = start_setup.elapsed();

    // -------------------------------------------------------------------------
    // Part 1
    // -------------------------------------------------------------------------
    // Count fuel to median position
    let start_part_1 = Instant::now();
    let fuel_1 = part_1(&positions)?;
    let time_part_1 = start_part_1.elapsed();

    // -------------------------------------------------------------------------
    // Part 2
    // -------------------------------------------------------------------------
    // Count fuel to average position
    let start_part_2 = Instant::now();
    let fuel_2 = part_2(&positions)?;
    let time_part_2 = start_part_2.elapsed();

    // -------------------------------------------------------------------------
    // Return
    // -------------------------------------------------------------------------
    Ok(RunData::new(
        fuel_1 as i64,
        fuel_2 as i64,
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
    output::print_day(7, "The Treachery of Whales")?;
    output::print_part(1, "🦀 Fuel", &format!("{}", run_data.part_1))?;
    output::print_part(2, "🦀 Fuel", &format!("{}", run_data.part_2))?;
    output::print_timing(&run_data.times)?;
    Ok(())
}

// -----------------------------------------------------------------------------
