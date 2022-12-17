#![feature(box_syntax)]
#![allow(dead_code)]
#![feature(option_result_contains)]
#![feature(entry_insert)]
#![feature(int_roundings)]

mod aocutil;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
// mod day18;
// mod day19;
// mod day20;
// mod day21;
// mod day22;
// mod day23;
// mod day24;
// mod day25;

const NUM_DAYS: u32 = 17;

use std::time::{Duration, Instant};

use aocutil::*;
use clap::{App, Arg};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

pub fn main() {
    let matches = App::new("AoC Runner")
        .version("2021")
        .author("Jesse Ashmore")
        .about("Advent of Code solution runner")
        .arg(
            Arg::new("day")
                .required(false)
                .takes_value(true)
                .about("Which solution to run, runs all solutions if omitted"),
        )
        .arg(
            Arg::new("language")
                .about("Target language for solution(s)")
                .short('l')
                .required(false)
                .default_value("rs"),
        )
        .arg(
            Arg::new("time")
                .about("Output solution(s) run time")
                .short('t')
                .long("time")
                .required(false)
                .takes_value(false),
        )
        .arg(
            Arg::new("sample")
                .about("Runs the sample input for the solution(s)")
                .short('s')
                .long("sample")
                .required(false)
                .takes_value(false),
        )
        .arg(
            Arg::new("parallel")
                .about("Runs all solutions in parallel")
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
        2 => run(day, day02::solve, sample),
        3 => run(day, day03::solve, sample),
        4 => run(day, day04::solve, sample),
        5 => run(day, day05::solve, sample),
        6 => run(day, day06::solve, sample),
        7 => run(day, day07::solve, sample),
        8 => run(day, day08::solve, sample),
        9 => run(day, day09::solve, sample),
        10 => run(day, day10::solve, sample),
        11 => run(day, day11::solve, sample),
        12 => run(day, day12::solve, sample),
        13 => run(day, day13::solve, sample),
        14 => run(day, day14::solve, sample),
        15 => run(day, day15::solve, sample),
        16 => run(day, day16::solve, sample),
        17 => run(day, day17::solve, sample),
        // 18 => run(day, day18::solve, sample),
        // 19 => run(day, day19::solve, sample),
        // 20 => run(day, day20::solve, sample),
        // 21 => run(day, day21::solve, sample),
        // 22 => run(day, day22::solve, sample),
        // 23 => run(day, day23::solve, sample),
        // 24 => run(day, day24::solve, sample),
        // 25 => run(day, day25::solve, sample),
        _ => panic!("Day not yet implemented"),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn all() {}
}

// TODO: Run Python solutions
// fn run_python(day: u32) {
//     let mut py = Command::new("python3");
//     py.arg(format!("inputs/day{day:02}.txt"));
// }
