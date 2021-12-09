//! Day 9:
//! Using the overset grid to homogonize operations helped efficiency today.
//! Without this overset, special logic would be needed to handle the edges and
//! corners of the smoke map. For the second part, I used a flow fill algorithm
//! changing the values so the recursion worked correctly.

use crate::prelude::*;

// -----------------------------------------------------------------------------
// Coordinate struct
// -----------------------------------------------------------------------------
#[derive(Default, Debug)]
struct Point {
    x: u8,
    y: u8,
}

// -----------------------------------------------------------------------------
// Part 1
// -----------------------------------------------------------------------------
fn part_1(smoke: &Vec<u8>, width: usize) -> crate::Result<(u32, Vec<Point>)> {
    let mut risk_sum = 0;
    let mut low_points = Vec::with_capacity(200);
    (1..width - 1).for_each(|j| {
        (1..width - 1).for_each(|i| {
            let current = smoke[i + j * width];
            if current < smoke[(i + 1) + j * width]
                && current < smoke[(i - 1) + j * width]
                && current < smoke[i + (j + 1) * width]
                && current < smoke[i + (j - 1) * width]
            {
                risk_sum += current as u32;
                low_points.push(Point {
                    x: i as u8,
                    y: j as u8,
                });
            }
        })
    });
    Ok((risk_sum, low_points))
}

// -----------------------------------------------------------------------------
// Part 2
// -----------------------------------------------------------------------------
fn find_basin_size(i: usize, j: usize, width: usize, smoke: &mut Vec<u8>) -> u32 {
    let current = smoke[i + j * width];
    if current == 10 {
        0
    } else {
        smoke[i + j * width] = 10;
        1 + find_basin_size(i + 1, j, width, smoke)
            + find_basin_size(i - 1, j, width, smoke)
            + find_basin_size(i, j + 1, width, smoke)
            + find_basin_size(i, j - 1, width, smoke)
    }
}

fn part_2(smoke: &mut Vec<u8>, low_points: Vec<Point>, width: usize) -> crate::Result<u32> {
    let mut basin_sizes: Vec<u32> = low_points
        .iter()
        .map(|point| find_basin_size(point.x as usize, point.y as usize, width, smoke))
        .collect();
    basin_sizes.sort_unstable_by(|a, b| b.cmp(a));
    Ok(basin_sizes.iter().take(3).product())
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
    let width = buffer.lines().nth(0).expect("failed to load input").len() + 2;
    let mut smoke = vec![10; width * width];
    buffer.lines().enumerate().for_each(|(j, line)| {
        line.as_bytes()
            .iter()
            .enumerate()
            .for_each(|(i, height)| smoke[(i + 1) + (j + 1) * width] = (*height - b'0') + 1)
    });
    let time_setup = start_setup.elapsed();

    // -------------------------------------------------------------------------
    // Part 1
    // -------------------------------------------------------------------------
    // Look for low points
    let start_part_1 = Instant::now();
    let (sum_1, low_points) = part_1(&smoke, width)?;
    let time_part_1 = start_part_1.elapsed();

    // -------------------------------------------------------------------------
    // Part 2
    // -------------------------------------------------------------------------
    // Count the size of each basin around a low point
    let start_part_2 = Instant::now();
    let product_2 = part_2(&mut smoke, low_points, width)?;
    let time_part_2 = start_part_2.elapsed();

    // -------------------------------------------------------------------------
    // Return
    // -------------------------------------------------------------------------
    Ok(RunData::new(
        sum_1 as i64,
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
    output::print_day(9, "Smoke Basin")?;
    output::print_part(1, "☁ Sum", &format!("{}", run_data.part_1))?;
    output::print_part(2, "☁ Size", &format!("{}", run_data.part_2))?;
    output::print_timing(&run_data.times)?;
    Ok(())
}

// -----------------------------------------------------------------------------
