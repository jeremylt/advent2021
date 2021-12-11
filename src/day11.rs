//! Day 11:

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
fn update(energy: &mut Vec<u8>, visits: &mut Vec<u16>, index: usize, current_visit: u16) -> u16 {
    if visits[index] == current_visit || visits[index] == 0 {
        return current_visit;
    };
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
fn count_flashes(energy: &mut Vec<u8>) -> i32 {
    let mut flash_count = 0;
    (1..WIDTH - 1).for_each(|j| {
        (1..WIDTH - 1).for_each(|i| {
            if energy[i + j * WIDTH] > 9 {
                energy[i + j * WIDTH] = 0;
                flash_count += 1;
            }
        })
    });
    flash_count as i32
}

// -----------------------------------------------------------------------------
// Part 1
// -----------------------------------------------------------------------------
fn part_1(energy: &mut Vec<u8>, visits: &mut Vec<u16>) -> crate::Result<i32> {
    let mut current_visit = 1;
    let mut flash_count = 0;
    (0..FIRST_GENERATIONS).for_each(|_| {
        (1..WIDTH - 1).for_each(|j| {
            (1..WIDTH - 1).for_each(|i| {
                energy[i+j*WIDTH] += 1;
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
    energy: &mut Vec<u8>,
    visits: &mut Vec<u16>,
    mut current_visit: u16,
) -> crate::Result<i32> {
    let mut generation = FIRST_GENERATIONS;
    let mut all_flashed = false;
    while !all_flashed {
        generation += 1;
        (1..WIDTH - 1).for_each(|j| {
            (1..WIDTH - 1).for_each(|i| {
                energy[i+j*WIDTH] += 1;
                current_visit = update(energy, visits, i + j * WIDTH, current_visit + 1);
            })
        });
        let flash_count = count_flashes(energy);
        all_flashed = flash_count as usize == (WIDTH - 2) * (WIDTH - 2);
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
    let mut energy: Vec<u8> = vec![0; WIDTH * WIDTH];
    let mut visits: Vec<u16> = vec![0; WIDTH * WIDTH];
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
    let increases_2 = part_2(&mut energy, &mut visits, count_1 as u16 * 10)?;
    let time_part_2 = start_part_2.elapsed();

    // -------------------------------------------------------------------------
    // Return
    // -------------------------------------------------------------------------
    Ok(RunData::new(
        count_1 as i64,
        increases_2 as i64,
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
    output::print_part(2, "🐙 Increase", &format!("{}", run_data.part_2))?;
    output::print_timing(&run_data.times)?;
    Ok(())
}

// -----------------------------------------------------------------------------
