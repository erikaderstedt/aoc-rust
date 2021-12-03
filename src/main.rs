use std::path::Path;
use std::time::{Instant,Duration};

use clap::App;
use clap::Arg;

mod crate_info;
mod days_2018;
mod days_2020;
mod days_2021;
mod common;
mod grid;

use crate::common::day_input_filename;
use crate::common::get_file_contents;
use crate::common::Solver;
use crate::crate_info::crate_author;
use crate::crate_info::crate_description;
use crate::crate_info::crate_name;
use crate::crate_info::crate_version;

#[macro_use]
extern crate itertools;

type SolverGetter = dyn Fn(u8) -> Option<Solver>;

fn main() -> Result<(), std::io::Error> {
    let cli = App::new(crate_name())
    .version(crate_version())
    .about(crate_description())
    .author(crate_author())
    .arg(
        Arg::with_name("year")
            .takes_value(true)
            .help(r#"Year (2015-2020) to run. If omitted, all years are run."#)
    )
    .arg(
        Arg::with_name("day")
            .takes_value(true)
            .help(r#"Day number (1 - 25) to run. If omitted, all days are run."#)
    );

    let matches = cli.get_matches();

    let years = match matches.value_of("year") {
        Some(year) => vec![year],
        _ => vec!["2018","2020","2021"],
    };
    let multiple_years = years.len() > 1;

    for year in years.into_iter() {
        if multiple_years { println!("============= {} =============", year); }

        let solver_getter = match year {
            "2018" => days_2018::get_solver,
            "2020" => days_2020::get_solver,
            "2021" => days_2021::get_solver,
            _ => panic!("Year not implemented!"),
        };

        let total_elapsed_time = match matches.value_of("day") {
            Some(day) => {
                let solver = solver_getter(day.parse::<u8>().unwrap_or_else(|_| panic!("Invalid day number: {}", day)));
                if let Some(s) = solver {
                    run_day(&s, year, day, matches.value_of("input-file").map(Path::new))
                } else {
                    panic!("No solver for that day!");
                }
            },
            _ => run_all_days(year, &solver_getter)
        }?;
        println!("Total elapsed time (generation + p1 + p2): {:?}", total_elapsed_time);
    }
    Ok(())    

}


fn run_day(day_func: &Solver, year: &str, day: &str, input_path: Option<&Path>) -> Result<Duration, std::io::Error> {
    print!("[Day {: >2}] ", day);

    let input = input_path
        .map(get_file_contents)
        .unwrap_or_else(|| get_file_contents(&day_input_filename(year,day)))?;

    //let s1 = day_func(&input);
    let start = Instant::now();        
    let solution = day_func(&input);
    let duration = start.elapsed();

    print!("A: {: <15} B: {: <50} ", solution.part_1,solution.part_2);
    println!("Elapsed time: {:>7} µs", duration.as_micros());

    Ok(duration)
}

fn run_all_days(year: &str, solver_getter: &SolverGetter) -> Result<Duration, std::io::Error> {
    (1..=25).filter_map(|day| 
        match solver_getter(day as u8) {
            Some(solver) => Some((solver, day)),
            None => None,
        }).map(|(solver, day)| run_day(&solver, year, &day.to_string(), None))
        .collect::<Result<Vec<Duration>,_>>()
        .map(|durations| durations.into_iter().sum())
}
