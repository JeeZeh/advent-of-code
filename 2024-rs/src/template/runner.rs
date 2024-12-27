/// Encapsulates code that interacts with solution functions.
use std::fmt::Display;
use std::hint::black_box;
use std::io::{stdout, Write};
use std::process::Output;
use std::time::{Duration, Instant};
use std::{cmp, env, process};

use crate::template::ANSI_BOLD;
use crate::template::{aoc_cli, Day, ANSI_ITALIC, ANSI_RESET};

pub type Solution<A, B> = (Option<A>, Option<B>);

pub fn run_solution<I: Clone, A: Display, B: Display>(func: impl Fn(I) -> Solution<A, B>, input: I, day: Day) {
    let (result, duration, samples) = run_timed(func, input, |_result| {});

    print_result(&result, &format_duration(&duration, samples));

    if let Some(part_one) = result.0 {
        submit_result(part_one, day, 1);
    }
    if let Some(part_two) = result.1 {
        submit_result(part_two, day, 2);
    }
}

/// Run a solution part. The behavior differs depending on whether we are running a release or debug build:
///  1. in debug, the function is executed once.
///  2. in release, the function is benched (approx. 1 second of execution time or 10 samples, whatever take longer.)
fn run_timed<I: Clone, A, B>(
    func: impl Fn(I) -> Solution<A, B>,
    input: I,
    hook: impl Fn(&Solution<A, B>),
) -> (Solution<A, B>, Duration, u128) {
    let timer = Instant::now();
    let result: Solution<A, B> = {
        let input = input.clone();

        #[cfg(feature = "dhat-heap")]
        let _profiler = dhat::Profiler::new_heap();

        func(input)
    };
    let base_time = timer.elapsed();

    hook(&result);

    let run = if std::env::args().any(|x| x == "--time") {
        bench(func, input, &base_time)
    } else {
        (base_time, 1)
    };

    (result, run.0, run.1)
}

fn bench<I: Clone, T>(func: impl Fn(I) -> T, input: I, base_time: &Duration) -> (Duration, u128) {
    let mut stdout = stdout();

    print!(" > {ANSI_ITALIC}benching{ANSI_RESET}");
    let _ = stdout.flush();

    let bench_iterations =
        (Duration::from_secs(1).as_nanos() / cmp::max(base_time.as_nanos(), 10)).clamp(10, 10000);

    let mut timers: Vec<Duration> = vec![];

    for _ in 0..bench_iterations {
        // need a clone here to make the borrow checker happy.
        let cloned = input.clone();
        let timer = Instant::now();
        black_box(func(black_box(cloned)));
        timers.push(timer.elapsed());
    }

    (
        #[allow(clippy::cast_possible_truncation)]
        Duration::from_nanos(average_duration(&timers) as u64),
        bench_iterations,
    )
}

fn average_duration(numbers: &[Duration]) -> u128 {
    numbers
        .iter()
        .map(std::time::Duration::as_nanos)
        .sum::<u128>()
        / numbers.len() as u128
}

fn format_duration(duration: &Duration, samples: u128) -> String {
    if samples == 1 {
        format!(" ({duration:.1?})")
    } else {
        format!(" ({duration:.1?} @ {samples} samples)")
    }
}

fn print_result<A: Display, B: Display>(result: &Solution<A, B>, duration_str: &str) {
    print!("\r");
    if let Some(part_one) = &result.0 {
        println!("Part 1: {ANSI_BOLD}{part_one}{ANSI_RESET}");
    } else {
        println!("Part 1: ✖");
    }
    if let Some(part_two) = &result.1 {
        println!("Part 2: {ANSI_BOLD}{part_two}{ANSI_RESET}");
    } else {
        println!("Part 2: ✖");
    }

    println!("Total duration: ▼ {duration_str}");
}

/// Parse the arguments passed to `solve` and try to submit one part of the solution if:
///  1. we are in `--release` mode.
///  2. aoc-cli is installed.
fn submit_result<T: Display>(
    result: T,
    day: Day,
    part: u8,
) -> Option<Result<Output, aoc_cli::AocCommandError>> {
    let args: Vec<String> = env::args().collect();

    if !args.contains(&"--submit".into()) {
        return None;
    }

    if args.len() < 2 {
        eprintln!("Unexpected command-line input. Format: cargo solve 1 --submit");
        process::exit(1);
    }

    if aoc_cli::check().is_err() {
        eprintln!("command \"aoc\" not found or not callable. Try running \"cargo install aoc-cli\" to install it.");
        process::exit(1);
    }

    println!("Submitting result via aoc-cli...");
    Some(aoc_cli::submit(day, part, &result.to_string()))
}
