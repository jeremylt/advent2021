// -----------------------------------------------------------------------------
// Modules
// -----------------------------------------------------------------------------
mod day01;
mod day02;
mod load;
mod output;

use crate::prelude::*;

// -----------------------------------------------------------------------------
// Error
// -----------------------------------------------------------------------------
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct Error {
    pub message: String,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl From<std::io::Error> for Error {
    fn from(io_error: std::io::Error) -> Self {
        Self {
            message: io_error.to_string(),
        }
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(parse_error: std::num::ParseIntError) -> Self {
        Self {
            message: parse_error.to_string(),
        }
    }
}

// -----------------------------------------------------------------------------
// RunData struct
// -----------------------------------------------------------------------------
#[derive(Debug)]
pub(crate) struct RunData {
    part_1: i64,
    part_2: i64,
    times: Timing,
}

impl RunData {
    fn new(part_1: i64, part_2: i64, times: Timing) -> Self {
        Self {
            part_1,
            part_2,
            times,
        }
    }
}

// -----------------------------------------------------------------------------
// Timing struct
// -----------------------------------------------------------------------------
#[derive(Debug)]
pub(crate) struct Timing {
    setup: std::time::Duration,
    part_1: std::time::Duration,
    part_2: std::time::Duration,
    combined: std::time::Duration,
}

impl Timing {
    fn new(
        setup: std::time::Duration,
        part_1: std::time::Duration,
        part_2: std::time::Duration,
        combined: std::time::Duration,
    ) -> Self {
        Self {
            setup,
            part_1,
            part_2,
            combined,
        }
    }
}

// -----------------------------------------------------------------------------
// Prelude
// -----------------------------------------------------------------------------
const REPETITIONS: u32 = 5;
pub(crate) mod prelude {
    pub(crate) use crate::REPETITIONS;
    pub(crate) use crate::{output, RunData, Timing};
    pub(crate) use colored::*;
    pub(crate) use std::fmt;
    pub(crate) use std::time::Instant;
}

// -----------------------------------------------------------------------------
// Main Driver
// -----------------------------------------------------------------------------
fn main() -> Result<()> {
    // Setup
    const DAYS: usize = 2;
    let runs = [day01::run, day02::run];
    let data = ["data/day01_actual.txt", "data/day02_actual.txt"];
    let reports = [day01::report, day02::report];

    // Each day
    output::print_header()?;
    let mut day_results: [Vec<RunData>; DAYS] = [vec![], vec![]];
    for _ in 0..REPETITIONS {
        for (i, day) in runs.iter().enumerate() {
            let buffer = crate::load::data_to_buffer(data[i].to_string())?;
            day_results[i].push(day(buffer)?);
        }
    }
    let average_times: Vec<Timing> = day_results
        .iter()
        .map(|day| {
            day.iter().fold(
                Timing::new(
                    std::time::Duration::new(0, 0),
                    std::time::Duration::new(0, 0),
                    std::time::Duration::new(0, 0),
                    std::time::Duration::new(0, 0),
                ),
                |acc, result| {
                    Timing::new(
                        acc.setup + result.times.setup / REPETITIONS,
                        acc.part_1 + result.times.part_1 / REPETITIONS,
                        acc.part_2 + result.times.part_2 / REPETITIONS,
                        acc.combined + result.times.combined / REPETITIONS,
                    )
                },
            )
        })
        .collect();
    for i in 0..DAYS {
        let result = day_results[i].first().ok_or(Error {
            message: format!("Day {} data not found", i),
        })?;
        let timing = &average_times[i];
        reports[i](&RunData::new(
            result.part_1,
            result.part_2,
            Timing::new(timing.setup, timing.part_1, timing.part_2, timing.combined),
        ))?;
    }

    // Day comparison
    output::print_header()?;
    let time_averages = average_times
        .iter()
        .map(|day| {
            if day.combined.as_nanos() > 1 {
                std::cmp::min(day.combined, day.setup + day.part_1 + day.part_2)
            } else {
                day.setup + day.part_1 + day.part_2
            }
        })
        .collect::<Vec<_>>();
    let time_std_devs: Vec<f64> = time_averages
        .iter()
        .zip(day_results.iter())
        .map(|(averages, day)| {
            (day.iter().fold(0.0, |acc, repetition| {
                let current = if repetition.times.combined.as_nanos() > 1 {
                    repetition.times.combined.as_nanos()
                } else {
                    repetition.times.setup.as_nanos()
                        + repetition.times.part_1.as_nanos()
                        + repetition.times.part_2.as_nanos()
                };
                acc + ((averages.as_nanos() as f64 - current as f64) / 1000.0).powf(2.0)
                    / ((REPETITIONS - 1) as f64)
            }))
            .powf(0.5)
        })
        .collect();
    output::print_days_timing(&time_averages, &time_std_devs)?;
    output::print_header()?;

    Ok(())
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    const MAX_TIME: u128 = 250;
    macro_rules! test_day {
        ($results:expr, $part_1:expr, $part_2:expr) => {
            assert_eq!($results.part_1, $part_1);
            assert_eq!($results.part_2, $part_2);
            assert!(
                ($results.times.setup + $results.times.part_1 + $results.times.part_2).as_millis()
                    < MAX_TIME
            );
        };
    }

    #[test]
    fn test_01_sample() -> Result<()> {
        let buffer = crate::load::data_to_buffer("data/day01_sample.txt".to_string())?;
        let results = day01::run(buffer)?;
        test_day!(results, 7, 5);
        Ok(())
    }

    #[test]
    fn test_01_actual() -> Result<()> {
        let buffer = crate::load::data_to_buffer("data/day01_actual.txt".to_string())?;
        let results = day01::run(buffer)?;
        test_day!(results, 1_228, 1_257);
        Ok(())
    }

    #[test]
    fn test_02_sample() -> Result<()> {
        let buffer = crate::load::data_to_buffer("data/day02_sample.txt".to_string())?;
        let results = day02::run(buffer)?;
        test_day!(results, 150, 900);
        Ok(())
    }

    #[test]
    fn test_02_actual() -> Result<()> {
        let buffer = crate::load::data_to_buffer("data/day02_actual.txt".to_string())?;
        let results = day02::run(buffer)?;
        test_day!(results, 1_714_680, 1_963_088_820);
        Ok(())
    }
}

// -----------------------------------------------------------------------------
