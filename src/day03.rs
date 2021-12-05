//! Day 3:
//! I'm not entirely happy with this solution because it feel very cumbersome.
//! The first part is messy because it is faster to count all bytes at the same
//! time. For the second part, partitioning seems the most straightforward
//! approach, but I have a lot of code duplication.

use crate::prelude::*;

// -----------------------------------------------------------------------------
// Part 1
// -----------------------------------------------------------------------------
fn part_1(report: &Vec<&str>) -> crate::Result<i64> {
    let num_entries = report.len();
    let entry_size = report[0].len();
    let counts = report.iter().fold(vec![0; entry_size], |mut acc, &curr| {
        curr.as_bytes()
            .iter()
            .enumerate()
            .for_each(|(i, b)| acc[i] += (b - b'0') as i64);
        acc
    });
    let counts_to_bytes: Vec<u8> = counts
        .iter()
        .map(|&count| {
            if count as usize > num_entries / 2 {
                b'1'
            } else {
                b'0'
            }
        })
        .collect();
    let most_common = i64::from_str_radix(&String::from_utf8(counts_to_bytes.clone())?, 2)?;
    let least_common = i64::from_str_radix(
        &String::from_utf8(
            counts_to_bytes
                .iter()
                .map(|&b| if b == b'0' { b'1' } else { b'0' })
                .collect(),
        )?,
        2,
    )?;
    Ok(most_common * least_common)
}

// -----------------------------------------------------------------------------
// Part 2
// -----------------------------------------------------------------------------
fn part_2(report: &Vec<&str>) -> crate::Result<i64> {
    let (ones, zeros): (Vec<&str>, Vec<&str>) =
        report.iter().partition(|curr| curr.as_bytes()[0] == b'1');
    let (mut most_common, mut least_common) = if zeros.len() > ones.len() {
        (zeros, ones)
    } else {
        (ones, zeros)
    };
    let mut i = 1;
    while most_common.len() > 1 {
        let (ones, zeros): (Vec<&str>, Vec<&str>) = most_common
            .iter()
            .partition(|curr| curr.as_bytes()[i] == b'1');
        most_common = if zeros.len() > ones.len() {
            zeros
        } else {
            ones
        };
        i += 1;
    }
    i = 1;
    while least_common.len() > 1 {
        let (ones, zeros): (Vec<&str>, Vec<&str>) = least_common
            .iter()
            .partition(|curr| curr.as_bytes()[i] == b'1');
        least_common = if ones.len() < zeros.len() || zeros.len() == 0 {
            ones
        } else {
            zeros
        };
        i += 1;
    }
    let most_common = i64::from_str_radix(most_common[0], 2)?;
    let least_common = i64::from_str_radix(least_common[0], 2)?;
    Ok(most_common * least_common)
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
    let report: Vec<&str> = buffer.lines().collect();
    let time_setup = start_setup.elapsed();

    // -------------------------------------------------------------------------
    // Part 1
    // -------------------------------------------------------------------------
    // Find most, least common bits represented across all values
    let start_part_1 = Instant::now();
    let product_1 = part_1(&report)?;
    let time_part_1 = start_part_1.elapsed();

    // -------------------------------------------------------------------------
    // Part 2
    // -------------------------------------------------------------------------
    // Find most, least common bits represented across reducing values
    let start_part_2 = Instant::now();
    let product_2 = part_2(&report)?;
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
    output::print_day(3, "Binary Diagnostic")?;
    output::print_part(1, "📄 Product", &format!("{}", run_data.part_1))?;
    output::print_part(2, "📄 Product", &format!("{}", run_data.part_2))?;
    output::print_timing(&run_data.times)?;
    Ok(())
}

// -----------------------------------------------------------------------------
