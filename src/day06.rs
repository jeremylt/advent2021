//! Day 6:
//! This was a reasonably straightforward problem. The key is to keep track of
//! number of fish of each age instead of modeling each fish separately.
//! Of note, manually rotating seems to be faster than `rotate_left(1)` in this
//! particular problem.

use crate::prelude::*;

const NUMBER_DAYS: usize = 9;
const NUMBER_GENERATIONS_SMALL: usize = 80;
const NUMBER_GENERATIONS_BIG: usize = 256;

// -----------------------------------------------------------------------------
// Part 1
// -----------------------------------------------------------------------------
fn part_1(population: &mut [usize; NUMBER_DAYS], generations: usize) -> crate::Result<usize> {
    (0..generations).for_each(|_| {
        let day_0 = population[0];
        (0..NUMBER_DAYS - 1).for_each(|i| population[i] = population[i + 1]);
        population[6] += day_0;
        population[8] = day_0;
    });
    Ok(population.iter().sum())
}

// -----------------------------------------------------------------------------
// Part 2
// -----------------------------------------------------------------------------
fn part_2(
    population: &mut [usize; NUMBER_DAYS],
    initial_generations: usize,
    final_generations: usize,
) -> crate::Result<usize> {
    (initial_generations..final_generations).for_each(|_| {
        let day_0 = population[0];
        (0..NUMBER_DAYS - 1).for_each(|i| population[i] = population[i + 1]);
        population[6] += day_0;
        population[8] = day_0;
    });
    Ok(population.iter().sum())
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
    let mut initial_population = [0; NUMBER_DAYS];
    buffer.trim().split(',').for_each(|age| {
        initial_population[age.parse::<usize>().expect("failed to parse fish")] += 1
    });
    let time_setup = start_setup.elapsed();

    // -------------------------------------------------------------------------
    // Part 1
    // -------------------------------------------------------------------------
    // Count intersections
    let start_part_1 = Instant::now();
    let count_1 = part_1(&mut initial_population, NUMBER_GENERATIONS_SMALL)?;
    let time_part_1 = start_part_1.elapsed();

    // -------------------------------------------------------------------------
    // Part 2
    // -------------------------------------------------------------------------
    // Count all intersections
    let start_part_2 = Instant::now();
    let count_2 = part_2(
        &mut initial_population,
        NUMBER_GENERATIONS_SMALL,
        NUMBER_GENERATIONS_BIG,
    )?;
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
    output::print_day(6, "Lanternfish")?;
    output::print_part(1, "🐟 Count", &format!("{}", run_data.part_1))?;
    output::print_part(2, "🐟 Count", &format!("{}", run_data.part_2))?;
    output::print_timing(&run_data.times)?;
    Ok(())
}

// -----------------------------------------------------------------------------
