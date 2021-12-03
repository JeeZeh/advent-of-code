mod aoc;
mod day01;
mod day02;

use aoc::*;
use clap::Parser;

#[derive(Parser)]
struct Opts {
    /// The day to run, if unspecified all days will be run
    day: Option<u32>,
    /// Language mode. Tries to run the given day in the language specified, if present. Defaults to Rust
    #[clap(default_value = "rs")]
    language: String,
}

pub fn main() {
    let opts: Opts = Opts::parse();

    if opts.day.is_some() {
        match opts.language.as_str() {
            "rs" => run_rust(opts.day.unwrap()),
            // "py" => run_python(opts.day),
            _ => panic!("Language not supported"),
        }
        .show();
    } else {
        for i in 1..=2 {
            println!("------------");
            println!("Day {}", i);
            println!("------------");
            run_rust(i).show();
        }
    }
}

fn run_rust(day: u32) -> Box<dyn AocOutput> {
    match day {
        1 => run(day, day01::solve),
        2 => run(day, day02::solve),
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
