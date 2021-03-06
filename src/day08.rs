//! Day 8:
//! For this puzzle I decided to parse all input into digits first. Pushing
//! all of the work into the input parsing made the two questions trivial.
//! To parse the final 4 digits, only the 1 and 4 of the initial digits are
//! required. You don't need to identify all 10 digits.

use crate::prelude::*;

const NUMBER_DIGITS: usize = 4;

//    0:      1:      2:      3:      4:
//  aaaa    ....    aaaa    aaaa    ....
// b    c  .    c  .    c  .    c  b    c
// b    c  .    c  .    c  .    c  b    c
//  ....    ....    dddd    dddd    dddd
// e    f  .    f  e    .  .    f  .    f
// e    f  .    f  e    .  .    f  .    f
//  gggg    ....    gggg    gggg    ....
//
//  5:      6:      7:      8:      9:
//  aaaa    aaaa    aaaa    aaaa    aaaa
// b    .  b    .  .    c  b    c  b    c
// b    .  b    .  .    c  b    c  b    c
//  dddd    dddd    ....    dddd    dddd
// .    f  e    f  .    f  e    f  .    f
// .    f  e    f  .    f  e    f  .    f
// gggg    gggg    ....    gggg    gggg
//
// 0: has 6 segments, 3 overlapping with 4
// 1: has 2 segments
// 2: has 5 segments, last option
// 3: has 5 segments, 2 overlapping with 1
// 4: has 4 segments
// 5: has 5 segments, 3 overlapping with 4
// 6: has 6 segments, 1 overlapping with 1
// 7: has 3 segments
// 8: has 7 segments
// 9: has 6 segments, last option

// -----------------------------------------------------------------------------
// Display data struct
// -----------------------------------------------------------------------------
#[derive(Default, Debug)]
struct Display {
    digits: [u8; NUMBER_DIGITS],
}

#[derive(Default, Debug)]
struct KeyDigits {
    one: u8,
    four: u8,
}

fn to_bits(s: &str) -> u8 {
    s.as_bytes()
        .iter()
        .fold(0_u8, |acc, b| acc + (1 << (b - b'a') as usize))
}

impl std::str::FromStr for Display {
    type Err = crate::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut data = s.splitn(2, '|');
        // Find identifier digits
        let digit_key = data
            .next()
            .expect("failed to split line")
            .splitn(11, ' ')
            .take(10)
            .fold(KeyDigits::default(), |mut key, digit| {
                match digit.len() {
                    2 => key.one = to_bits(digit),
                    4 => key.four = to_bits(digit),
                    _ => (),
                };
                key
            });

        // Convert final 4 digits
        let digits = data
            .next()
            .expect("failed to split line")
            .splitn(5, ' ')
            .skip(1)
            .take(4)
            .enumerate()
            .fold([0; NUMBER_DIGITS], |mut digits, (i, digit)| {
                let value = to_bits(digit);
                digits[i] = match digit.len() {
                    2 => 1,
                    3 => 7,
                    4 => 4,
                    7 => 8,
                    5 => {
                        if (value & digit_key.one).count_ones() == 2 {
                            3
                        } else if (value & digit_key.four).count_ones() == 3 {
                            5
                        } else {
                            2
                        }
                    }
                    6 => {
                        if (value & digit_key.one).count_ones() == 1 {
                            6
                        } else if (value & digit_key.four).count_ones() == 3 {
                            0
                        } else {
                            9
                        }
                    }
                    _ => unreachable!(),
                };
                digits
            });
        Ok(Self { digits })
    }
}

impl Display {
    fn count_simple_digits(&self) -> u8 {
        self.digits
            .iter()
            .filter(|digit| [1, 4, 7, 8].contains(*digit))
            .count() as u8
    }

    fn digit_sum(&self) -> u32 {
        self.digits
            .iter()
            .fold(0, |sum, curr| sum * 10 + *curr as u32)
    }
}

// -----------------------------------------------------------------------------
// Part 1
// -----------------------------------------------------------------------------
fn part_1(positions: &Vec<Display>) -> crate::Result<u32> {
    Ok(positions
        .iter()
        .map(|display| display.count_simple_digits() as u32)
        .sum())
}

// -----------------------------------------------------------------------------
// Part 2
// -----------------------------------------------------------------------------
fn part_2(positions: &Vec<Display>) -> crate::Result<u32> {
    Ok(positions.iter().map(|display| display.digit_sum()).sum())
}

// -----------------------------------------------------------------------------
// Combined
// -----------------------------------------------------------------------------
fn combined(positions: &mut dyn Iterator<Item = Display>) -> crate::Result<(u32, u32)> {
    Ok(positions.fold((0, 0), |acc, display| {
        (
            acc.0 + display.count_simple_digits() as u32,
            acc.1 + display.digit_sum(),
        )
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
    // Combined
    // -------------------------------------------------------------------------
    let start_combined = Instant::now();
    let mut displays = buffer
        .lines()
        .map(|line| line.parse().expect("failed to decode line"));
    let (count_combined, sum_combined) = combined(&mut displays)?;
    let time_combined = start_combined.elapsed();
    assert_eq!(count_1, count_combined);
    assert_eq!(sum_2, sum_combined);

    // -------------------------------------------------------------------------
    // Return
    // -------------------------------------------------------------------------
    Ok(RunData::new(
        count_1 as i64,
        sum_2 as i64,
        Timing::new(time_setup, time_part_1, time_part_2, time_combined),
    ))
}

// -----------------------------------------------------------------------------
// Report
// -----------------------------------------------------------------------------
pub(crate) fn report(run_data: &RunData) -> crate::Result<()> {
    output::print_day(8, "Seven Segment Search")?;
    output::print_part(1, "???? Count", &format!("{}", run_data.part_1))?;
    output::print_part(2, "???? Sum", &format!("{}", run_data.part_2))?;
    output::print_timing(&run_data.times)?;
    Ok(())
}

// -----------------------------------------------------------------------------
