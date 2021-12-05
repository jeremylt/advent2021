//! Day 5:
//! This solution process highlighted two things. First, branching logic can
//! be expensive. My original solution to part 1 was 3x slower than the
//! solution to part 2, but the only difference was checking if `dx` or `dy`
//! was zero. Second, this difference reduced greatly when I switched to a
//! narrower data type, using `i/u16` instead of `i/u32` and using a `u8` for
//! my `grid`s representing the vents.

use crate::prelude::*;

const GRID_SIZE: usize = 1000;

// -----------------------------------------------------------------------------
// Segment data struct
// -----------------------------------------------------------------------------
#[derive(Default, Debug)]
struct Point {
    x: u16,
    y: u16,
}

#[derive(Default, Debug)]
struct Segment {
    start: Point,
    stop: Point,
    dx: i16,
    dy: i16,
    steps: u16,
}

impl std::str::FromStr for Segment {
    type Err = crate::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut line = s.splitn(4, &[',', '-'][..]);
        let start_x = line.next().expect("failed to parse segment").parse()?;
        let start_y = line
            .next()
            .expect("failed to parse segment")
            .trim()
            .parse()?;
        let stop_x = line.next().expect("failed to parse segment")[2..]
            .trim()
            .parse()?;
        let stop_y = line.next().expect("failed to parse segment").parse()?;
        let dx = if start_x == stop_x {
            0
        } else if stop_x > start_x {
            1
        } else {
            -1
        };
        let dy = if start_y == stop_y {
            0
        } else if stop_y > start_y {
            1
        } else {
            -1
        };
        let steps = std::cmp::max(
            (start_x as i16 - stop_x as i16).abs(),
            (start_y as i16 - stop_y as i16).abs(),
        ) as u16;
        Ok(Self {
            start: Point {
                x: start_x,
                y: start_y,
            },
            stop: Point {
                x: stop_x,
                y: stop_y,
            },
            dx: dx,
            dy: dy,
            steps: steps,
        })
    }
}

// -----------------------------------------------------------------------------
// Part 1
// -----------------------------------------------------------------------------
fn part_1(
    grid: &mut [[u8; GRID_SIZE]; GRID_SIZE],
    segments: &Vec<Segment>,
) -> crate::Result<usize> {
    segments
        .iter()
        .filter(|segment| segment.dx == 0 || segment.dy == 0)
        .for_each(|segment| {
            (0..=segment.steps).for_each(|step| {
                grid[(segment.start.x as i16 + step as i16 * segment.dx) as usize]
                    [(segment.start.y as i16 + step as i16 * segment.dy) as usize] += 1
            })
        });
    let count = grid
        .iter()
        .map(|row| row.iter().filter(|count| **count > 1).count())
        .sum();
    Ok(count)
}

// -----------------------------------------------------------------------------
// Part 2
// -----------------------------------------------------------------------------
fn part_2(
    grid: &mut [[u8; GRID_SIZE]; GRID_SIZE],
    segments: &Vec<Segment>,
) -> crate::Result<usize> {
    segments
        .iter()
        .filter(|segment| segment.dx != 0 && segment.dy != 0)
        .for_each(|segment| {
            (0..=segment.steps).for_each(|step| {
                grid[(segment.start.x as i16 + step as i16 * segment.dx) as usize]
                    [(segment.start.y as i16 + step as i16 * segment.dy) as usize] += 1
            })
        });
    let count = grid
        .iter()
        .map(|row| row.iter().filter(|count| **count > 1).count())
        .sum();
    Ok(count)
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
    let segments: Vec<Segment> = buffer
        .lines()
        .map(|line| line.parse().expect("failed to parse line"))
        .collect();
    let time_setup = start_setup.elapsed();

    // -------------------------------------------------------------------------
    // Part 1
    // -------------------------------------------------------------------------
    // Count intersections
    let start_part_1 = Instant::now();
    let mut grid = [[0_u8; GRID_SIZE]; GRID_SIZE];
    let count_1 = part_1(&mut grid, &segments)?;
    let time_part_1 = start_part_1.elapsed();

    // -------------------------------------------------------------------------
    // Part 2
    // -------------------------------------------------------------------------
    // Count all intersections
    let start_part_2 = Instant::now();
    let count_2 = part_2(&mut grid, &segments)?;
    let time_part_2 = start_part_2.elapsed();

    // -------------------------------------------------------------------------
    // Return
    // -------------------------------------------------------------------------
    Ok(RunData::new(
        count_1 as i64,
        count_2 as i64,
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
    output::print_day(5, "Hydrothermal Venture")?;
    output::print_part(1, "🪨 Count", &format!("{}", run_data.part_1))?;
    output::print_part(2, "🪨 Count", &format!("{}", run_data.part_2))?;
    output::print_timing(&run_data.times)?;
    Ok(())
}

// -----------------------------------------------------------------------------
