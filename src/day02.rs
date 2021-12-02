//! Day 2:
//! The main part of the effort for this solution is reading the input into a
//! correct structure. From there folowing the computations are straightforward.
//! In the second part, I leveraged the fact that one of the two parts of the
//! Direction is zero to avoid branching logic, making the code take half as
//! long as the branching version of the `fold`.

use crate::prelude::*;

// -----------------------------------------------------------------------------
// Direction data struct
// -----------------------------------------------------------------------------
#[derive(Debug)]
struct Direction {
    horizontal: i32, // positive is forward
    vertical: i32,   // positive is down
}

impl std::str::FromStr for Direction {
    type Err = crate::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut line = s.splitn(2, ' ');
        match line.next().expect("failed to parse command") {
            "forward" => Ok(Self {
                horizontal: s[8..].parse()?,
                vertical: 0,
            }),
            "down" => Ok(Self {
                horizontal: 0,
                vertical: s[5..].parse()?,
            }),
            "up" => Ok(Self {
                horizontal: 0,
                vertical: -s[3..].parse()?,
            }),
            _ => Err(crate::Error {
                message: "invalid direction".to_string(),
            }),
        }
    }
}

// -----------------------------------------------------------------------------
// Direction with Aim data struct
// -----------------------------------------------------------------------------
#[derive(Debug)]
struct AimedDirection {
    aim: i32,
    horizontal: i32, // positive is forward
    vertical: i32,   // positive is down
}

// -----------------------------------------------------------------------------
// Part 1
// -----------------------------------------------------------------------------
fn part_1(directions: &Vec<Direction>) -> crate::Result<Direction> {
    Ok(directions.iter().fold(
        Direction {
            horizontal: 0,
            vertical: 0,
        },
        |mut acc, curr| {
            acc.horizontal += curr.horizontal;
            acc.vertical += curr.vertical;
            acc
        },
    ))
}

// -----------------------------------------------------------------------------
// Part 2
// -----------------------------------------------------------------------------
fn part_2(directions: &Vec<Direction>) -> crate::Result<AimedDirection> {
    Ok(directions.iter().fold(
        AimedDirection {
            aim: 0,
            horizontal: 0,
            vertical: 0,
        },
        |mut acc, curr| {
            // One direction is aways zero, so can safely do both paths
            acc.aim += curr.vertical; // increase aim by Y
            acc.horizontal += curr.horizontal; // increase horizontal by X
            acc.vertical += acc.aim * curr.horizontal; // increase depth by aim * X
            acc
        },
    ))
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
    let directions: Vec<Direction> = buffer
        .lines()
        .map(|line| line.parse().expect("failed to parse line"))
        .collect();
    let time_setup = start_setup.elapsed();

    // -------------------------------------------------------------------------
    // Part 1
    // -------------------------------------------------------------------------
    // Sum coordinates
    let start_part_1 = Instant::now();
    let directions_1 = part_1(&directions)?;
    let product_1 = directions_1.horizontal * directions_1.vertical;
    let time_part_1 = start_part_1.elapsed();

    // -------------------------------------------------------------------------
    // Part 2
    // -------------------------------------------------------------------------
    // Compute coordinates with aimed directions
    let start_part_2 = Instant::now();
    let directions_2 = part_2(&directions)?;
    let product_2 = directions_2.horizontal * directions_2.vertical;
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
    output::print_day(1, "Dive!")?;
    output::print_part(1, "🧭 Product", &format!("{}", run_data.part_1))?;
    output::print_part(2, "🧭 Product", &format!("{}", run_data.part_2))?;
    output::print_timing(&run_data.times)?;
    Ok(())
}

// -----------------------------------------------------------------------------
