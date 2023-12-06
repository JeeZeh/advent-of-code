#![allow(dead_code)]
#![feature(entry_insert)]
#![feature(int_roundings)]

mod aocutil;
mod day01;

const NUM_DAYS: u32 = 1;

use std::time::{Duration, Instant};

use aocutil::*;
use clap::{App, Arg};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

pub fn main() {
    let matches = App::new("AoC Runner")
        .version("2023")
        .author("Jesse Ashmore")
        .about("Advent of Code solution runner")
        .arg(
            Arg::new("day")
                .required(false)
                .takes_value(true)
                .help("Which solution to run, runs all solutions if omitted"),
        )
        .arg(
            Arg::new("language")
                .help("Target language for solution(s)")
                .short('l')
                .required(false)
                .default_value("rs"),
        )
        .arg(
            Arg::new("time")
                .help("Output solution(s) run time")
                .short('t')
                .long("time")
                .required(false)
                .takes_value(false),
        )
        .arg(
            Arg::new("sample")
                .help("Runs the sample input for the solution(s)")
                .short('s')
                .long("sample")
                .required(false)
                .takes_value(false),
        )
        .arg(
            Arg::new("parallel")
                .help("Runs all solutions in parallel")
                .short('p')
                .long("parallel")
                .required(false)
                .takes_value(false),
        )
        .get_matches();

    let time = matches.is_present("time");
    let sample_test = matches.is_present("sample");
    let parallel = matches.is_present("parallel");

    if let Some(day) = matches.value_of("day") {
        let (output, dur) = match matches.value_of("language").unwrap() {
            "rs" => run_rust(day.parse().unwrap(), sample_test),
            // "py" => run_python(opts.day),
            _ => panic!("Language not supported"),
        };
        output.show();
        if time {
            println!("Time: {:.2?}", dur);
        }
        return;
    }

    let mut total = Duration::new(0, 0);

    if parallel {
        let now = Instant::now();
        (1..=NUM_DAYS).into_par_iter().for_each(|i| {
            run_rust(i, sample_test);
        });
        total += now.elapsed();
    } else {
        for i in 1..=NUM_DAYS {
            let (output, duration) = run_rust(i, sample_test);
            println!("------------");
            if time {
                println!("Day {} — {:.2?}", i, duration);
                total += duration;
            } else {
                println!("Day {}", i);
            }
            println!("------------");
            output.show();
            println!();
        }
    }
    if time {
        println!("------------");
        println!("Total Execution Time: {:.2?}", total);
    }
}

fn run_rust(day: u32, sample: bool) -> (Box<dyn AocOutput>, Duration) {
    match day {
        1 => run(day, day01::solve, sample),
        _ => panic!("Day not yet implemented"),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn all() {}
}
