//! Day 11:
//! This solution relies upon knowledge of the grid size for performance.
//! Without this knowledge, the grids need to be stored in Vecs instead
//! of arrays. I used a recursive update around each point that triggered
//! a flash. Most of the speedup came from homogonizing loops and
//! reducing branching logic.

use crate::prelude::*;

const WIDTH: usize = 12;
const FIRST_GENERATIONS: usize = 100;
const OFFSETS: [i16; 8] = [
    0 - WIDTH as i16,
    0 + WIDTH as i16,
    1,
    1 - WIDTH as i16,
    1 + WIDTH as i16,
    -1,
    -1 - WIDTH as i16,
    -1 + WIDTH as i16,
];

// -----------------------------------------------------------------------------
// Update energy state
// -----------------------------------------------------------------------------
fn update(
    energy: &mut [u8; WIDTH * WIDTH],
    visits: &mut [u16; WIDTH * WIDTH],
    index: usize,
    current_visit: u16,
) -> u16 {
    visits[index] = current_visit;

    if energy[index] == 10 {
        OFFSETS.iter().for_each(|offset| {
            visits[index] += 1;
            energy[(index as i16 + offset) as usize] += 1;
            visits[index] = update(
                energy,
                visits,
                (index as i16 + offset) as usize,
                visits[index],
            );
        });
    }

    visits[index]
}

// -----------------------------------------------------------------------------
// Count flashes
// -----------------------------------------------------------------------------
fn count_flashes(energy: &mut [u8; WIDTH * WIDTH]) -> i32 {
    let flash_count = (1..WIDTH - 1).fold(0, |acc, j| {
        acc + (1..WIDTH - 1).fold(0, |acc, i| {
            if energy[i + j * WIDTH] > 9 {
                energy[i + j * WIDTH] = 0;
                acc + 1
            } else {
                acc
            }
        })
    });
    flash_count as i32
}

// -----------------------------------------------------------------------------
// Part 1
// -----------------------------------------------------------------------------
fn part_1(
    energy: &mut [u8; WIDTH * WIDTH],
    visits: &mut [u16; WIDTH * WIDTH],
) -> crate::Result<i32> {
    let mut current_visit = 1;
    let mut flash_count = 0;
    (0..FIRST_GENERATIONS).for_each(|_| {
        (1..WIDTH - 1).for_each(|j| {
            (1..WIDTH - 1).for_each(|i| {
                energy[i + j * WIDTH] += 1;
                current_visit = update(energy, visits, i + j * WIDTH, current_visit + 1);
            })
        });
        flash_count += count_flashes(energy);
    });
    Ok(flash_count as i32)
}

// -----------------------------------------------------------------------------
// Part 2
// -----------------------------------------------------------------------------
fn part_2(
    energy: &mut [u8; WIDTH * WIDTH],
    visits: &mut [u16; WIDTH * WIDTH],
    mut current_visit: u16,
) -> crate::Result<i32> {
    let mut generation = FIRST_GENERATIONS;
    let mut all_flashed = false;
    while !all_flashed {
        generation += 1;
        (1..WIDTH - 1).for_each(|j| {
            (1..WIDTH - 1).for_each(|i| {
                energy[i + j * WIDTH] += 1;
                current_visit = update(energy, visits, i + j * WIDTH, current_visit + 1);
            })
        });
        all_flashed = count_flashes(energy) as usize == (WIDTH - 2) * (WIDTH - 2);
    }
    Ok(generation as i32)
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
    let mut energy: [u8; WIDTH * WIDTH] = [11; WIDTH * WIDTH];
    let mut visits: [u16; WIDTH * WIDTH] = [0; WIDTH * WIDTH];
    buffer.lines().enumerate().for_each(|(j, line)| {
        line.as_bytes().iter().enumerate().for_each(|(i, b)| {
            energy[(i + 1) + (j + 1) * WIDTH] = *b - b'0';
            visits[(i + 1) + (j + 1) * WIDTH] = 1;
        })
    });
    let time_setup = start_setup.elapsed();

    // -------------------------------------------------------------------------
    // Part 1
    // -------------------------------------------------------------------------
    // Run 100 generations
    let start_part_1 = Instant::now();
    let count_1 = part_1(&mut energy, &mut visits)?;
    let time_part_1 = start_part_1.elapsed();

    // -------------------------------------------------------------------------
    // Part 2
    // -------------------------------------------------------------------------
    // Look for syncronization
    let start_part_2 = Instant::now();
    let generation_2 = part_2(&mut energy, &mut visits, count_1 as u16 * 10)?;
    let time_part_2 = start_part_2.elapsed();

    // -------------------------------------------------------------------------
    // Return
    // -------------------------------------------------------------------------
    Ok(RunData::new(
        count_1 as i64,
        generation_2 as i64,
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
    output::print_day(11, "Dumbo Octopus")?;
    output::print_part(1, "🐙 Count", &format!("{}", run_data.part_1))?;
    output::print_part(2, "🐙 Generation", &format!("{}", run_data.part_2))?;
    output::print_timing(&run_data.times)?;
    Ok(())
}

// -----------------------------------------------------------------------------
