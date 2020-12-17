use std::path::Path;
use std::time::{Instant,Duration};

use clap::App;
use clap::Arg;

mod crate_info;
mod days;
mod common;
mod grid;

use crate::common::day_input_filename;
use crate::common::get_file_contents;
use crate::crate_info::crate_author;
use crate::crate_info::crate_description;
use crate::crate_info::crate_name;
use crate::crate_info::crate_version;

#[macro_use]
extern crate itertools;

fn main() -> Result<(), std::io::Error> {
    let cli = App::new(crate_name())
    .version(crate_version())
    .about(crate_description())
    .author(crate_author())
    .arg(
        Arg::with_name("day")
            .takes_value(true)
            .help(r#"Day number (1 - 25) to run. If omitted, all days are run."#)
    );

    // TODO: run all days listed. If no days are listed, run all days which have solvers.

    let matches = cli.get_matches();

    let total_elapsed_time = match matches.value_of("day") {
        Some(day) => run_day(day.parse::<u8>()
                        .unwrap_or_else(|_| panic!(format!("Invalid day number: {}", day))),
                        matches.value_of("input-file").map(Path::new),
                    ),
        _ => run_all_days()
    }?;
    println!("Total elapsed time (generation + p1 + p2): {:?}", total_elapsed_time);
    Ok(())
}


fn run_day(day: u8, input_path: Option<&Path>) -> Result<Duration, std::io::Error> {
    print!("=== Day {: >2} ===> ", day);

    let day_func = days::get_solver(day).unwrap_or_else(|| panic!(format!("Unknown day: {}", day)));

    let input = input_path
        .map(get_file_contents)
        .unwrap_or_else(|| get_file_contents(&day_input_filename(day)))?;

    //let s1 = day_func(&input);
    let start = Instant::now();        
    let solution = day_func(&input);
    let duration = start.elapsed();

    print!("A: {: <10}\t\tB: {: <16}\t", solution.part_1,solution.part_2);
    println!("Elapsed time: {:#?}", duration);

    Ok(duration)
}

fn run_all_days() -> Result<Duration, std::io::Error> {
    days::all_numbers()
        .iter()
        .map(|&day| run_day(day, None))
        .collect::<Result<Vec<Duration>,_>>()
        .map(|durations| durations.into_iter().sum())
}
