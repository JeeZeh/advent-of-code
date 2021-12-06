#![feature(box_syntax)]
#![allow(dead_code)]

mod aoc;
mod aocutil;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;

use std::time::Duration;

use aoc::*;
use clap::{App, Arg};

pub fn main() {
    let matches = App::new("AoC Runner")
        .version("2021")
        .author("Jesse Ashmore")
        .about("Advent of Code solution runner")
        .arg(
            Arg::new("day")
                .required(false)
                .takes_value(true)
                .about("Which solution to run, runs all solutions if ommited"),
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
        .get_matches();

    let time = matches.is_present("time");

    if let Some(day) = matches.value_of("day") {
        let (output, dur) = match matches.value_of("language").unwrap() {
            "rs" => run_rust(day.parse().unwrap()),
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
    for i in 1..=6 {
        println!("------------");
        println!("Day {}", i);
        println!("------------");
        let (output, duration) = run_rust(i);
        output.show();
        if time {
            println!("Time: {:.2?}", duration);
            total += duration;
        }
    }

    if time {
        println!("------------");
        println!("Total Execution Time: {:.2?}", total);
    }
}

fn run_rust(day: u32) -> (Box<dyn AocOutput>, Duration) {
    match day {
        1 => run(day, day01::solve),
        2 => run(day, day02::solve),
        3 => run(day, day03::solve),
        4 => run(day, day04::solve),
        5 => run(day, day05::solve),
        6 => run(day, day06::solve),
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
