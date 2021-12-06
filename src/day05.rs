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
    x: i16,
    y: i16,
}

#[derive(Default, Debug)]
struct Segment {
    start: Point,
    stop: Point,
    dx: i8,
    dy: i8,
    steps: i16,
}

impl std::str::FromStr for Segment {
    type Err = crate::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut line = s.split(|c| !char::is_numeric(c));
        let start_x = line.next().expect("failed to parse segment").parse()?;
        let start_y = line.next().expect("failed to parse segment").parse()?;
        line.next();
        line.next();
        line.next();
        let stop_x = line.next().expect("failed to parse segment").parse()?;
        let stop_y = line.next().expect("failed to parse segment").parse()?;
        let dx = (stop_x as i16 - start_x as i16).signum() as i8;
        let dy = (stop_y as i16 - start_y as i16).signum() as i8;
        let steps = std::cmp::max(
            (start_x as i16 - stop_x as i16).abs(),
            (start_y as i16 - stop_y as i16).abs(),
        );
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
// Part 1/2
// -----------------------------------------------------------------------------
fn part_1(grid: &mut Vec<u8>, segments: &Vec<Segment>) -> crate::Result<usize> {
    let mut count = 0;
    segments.iter().for_each(|segment| {
        let start = (segment.start.x as usize) * GRID_SIZE + segment.start.y as usize;
        let stop = (segment.stop.x as usize) * GRID_SIZE + segment.stop.y as usize;
        let step = (segment.dx as i32 * GRID_SIZE as i32 + segment.dy as i32).abs() as usize;
        let (a, b) = if start < stop {
            (start, stop)
        } else {
            (stop, start)
        };
        (a..=b).step_by(step).for_each(|index| unsafe {
            let point = grid.get_unchecked_mut(index);
            *point += 1;
            if *point == 2 {
                count += 1;
            }
        })
    });
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
    let mut segments_1: Vec<Segment> = Vec::new();
    let mut segments_2: Vec<Segment> = Vec::new();
    buffer.lines().for_each(|line| {
        let segment: Segment = line.parse().expect("failed to parse line");
        if segment.dx == 0 || segment.dy == 0 {
            segments_1.push(segment)
        } else {
            segments_2.push(segment)
        };
    });
    let mut grid = vec![0_u8; GRID_SIZE * GRID_SIZE];
    let time_setup = start_setup.elapsed();

    // -------------------------------------------------------------------------
    // Part 1
    // -------------------------------------------------------------------------
    // Count intersections
    let start_part_1 = Instant::now();
    let count_1 = part_1(&mut grid, &segments_1)?;
    let time_part_1 = start_part_1.elapsed();

    // -------------------------------------------------------------------------
    // Part 2
    // -------------------------------------------------------------------------
    // Count all intersections
    let start_part_2 = Instant::now();
    let count_2 = count_1 + part_1(&mut grid, &segments_2)?;
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
    output::print_part(1, "🐙 Count", &format!("{}", run_data.part_1))?;
    output::print_part(2, "🐙 Count", &format!("{}", run_data.part_2))?;
    output::print_timing(&run_data.times)?;
    Ok(())
}

// -----------------------------------------------------------------------------
