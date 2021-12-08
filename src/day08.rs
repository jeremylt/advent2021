//! Day 8:
//! For this puzzle I decided to parse all input into digits first. Pushing
//! all of the work into the input parsing made the two questions trivial.

use crate::prelude::*;

const NUMBER_DIGITS_DISPLAY: usize = 4;
const NUMBER_DIGITS_TOTAL: usize = 10;

// -----------------------------------------------------------------------------
// Display data struct
// -----------------------------------------------------------------------------
#[derive(Default, Debug)]
struct Display {
    digits: [u8; NUMBER_DIGITS_DISPLAY],
}

fn to_bits(s: &str) -> u8 {
    s.as_bytes()
        .iter()
        .fold(0_u8, |acc, b| acc + (1 << (b - b'a') as usize))
}

impl std::str::FromStr for Display {
    type Err = crate::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Convert first 10 digits
        let mut digit_key = [0; NUMBER_DIGITS_TOTAL];
        let mut line = s.splitn(11, ' ');
        line.by_ref().take(10).for_each(|digit| match digit.len() {
            2 => digit_key[1] = to_bits(digit),
            3 => digit_key[7] = to_bits(digit),
            4 => digit_key[4] = to_bits(digit),
            7 => digit_key[8] = 127,
            _ => (),
        });
        let mut line = s.splitn(16, &[' ', '|'][..]);
        line.by_ref().take(10).for_each(|digit| match digit.len() {
            5 => {
                let value = to_bits(digit);
                if (value & digit_key[1]).count_ones() == 2 {
                    digit_key[3] = value;
                } else if (value & (digit_key[4] & !digit_key[1])).count_ones() == 2 {
                    digit_key[5] = value;
                } else {
                    digit_key[2] = value;
                }
            }
            6 => {
                let value = to_bits(digit);
                if (value & digit_key[1]).count_ones() == 1 {
                    digit_key[6] = value;
                } else if (value & (digit_key[4] & !digit_key[1])).count_ones() == 1 {
                    digit_key[0] = value;
                } else {
                    digit_key[9] = value;
                }
            }
            _ => (),
        });
        line.next();
        line.next();
        // Convert final 4 digits
        let mut digits = [0_u8; NUMBER_DIGITS_DISPLAY];
        line.take(4).enumerate().for_each(|(i, digit)| {
            let value = to_bits(digit);
            digits[i] = digit_key
                .iter()
                .position(|&num| num == value)
                .expect("failed to find digit") as u8;
        });
        Ok(Self { digits })
    }
}

// -----------------------------------------------------------------------------
// Part 1
// -----------------------------------------------------------------------------
fn part_1(positions: &Vec<Display>) -> crate::Result<u32> {
    Ok(positions
        .iter()
        .map(|display| {
            display
                .digits
                .iter()
                .filter(|&&digit| digit == 1 || digit == 4 || digit == 7 || digit == 8)
                .count() as u32
        })
        .sum())
}

// -----------------------------------------------------------------------------
// Part 2
// -----------------------------------------------------------------------------
fn part_2(positions: &Vec<Display>) -> crate::Result<u32> {
    Ok(positions
        .iter()
        .map(|display| {
            display.digits[0] as u32 * 1000
                + display.digits[1] as u32 * 100
                + display.digits[2] as u32 * 10
                + display.digits[3] as u32
        })
        .sum())
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
    let displays: Vec<Display> = buffer
        .lines()
        .map(|line| line.parse().expect("failed to decode line"))
        .collect();
    let time_setup = start_setup.elapsed();

    // -------------------------------------------------------------------------
    // Part 1
    // -------------------------------------------------------------------------
    // Count 'easy' digits
    let start_part_1 = Instant::now();
    let count_1 = part_1(&displays)?;
    let time_part_1 = start_part_1.elapsed();

    // -------------------------------------------------------------------------
    // Part 2
    // -------------------------------------------------------------------------
    // Sum all digits
    let start_part_2 = Instant::now();
    let sum_2 = part_2(&displays)?;
    let time_part_2 = start_part_2.elapsed();

    // -------------------------------------------------------------------------
    // Return
    // -------------------------------------------------------------------------
    Ok(RunData::new(
        count_1 as i64,
        sum_2 as i64,
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
    output::print_day(8, "Seven Segment Search")?;
    output::print_part(1, "🔢 Count", &format!("{}", run_data.part_1))?;
    output::print_part(2, "🔢 Sum", &format!("{}", run_data.part_2))?;
    output::print_timing(&run_data.times)?;
    Ok(())
}

// -----------------------------------------------------------------------------
