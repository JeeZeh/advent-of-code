use chrono::prelude::*;
use std::{collections::HashMap, fs, str::FromStr, string::ParseError, time::Instant};

#[derive(Debug)]

enum EntryType {
    Wake,
    Sleep,
    Rotate,
}

type Guard = Option<String>;

#[derive(Debug)]
struct Entry {
    entry_type: EntryType,
    timestamp: DateTime<Utc>,
    guard: Guard,
}

impl FromStr for Entry {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s[1..].split("] ");
        let timestamp = Utc
            .datetime_from_str(parts.next().unwrap(), "%Y-%m-%d %H:%M")
            .unwrap();

        let entry = parts.next().unwrap();
        let entry_type: EntryType;
        let mut guard: Option<String> = None;

        if entry.contains("wake") {
            entry_type = EntryType::Wake;
        } else if entry.contains("sleep") {
            entry_type = EntryType::Sleep;
        } else {
            entry_type = EntryType::Rotate;
            guard = Some(String::from(&entry.split(" ").nth(1).unwrap()[1..]));
        }

        Ok(Entry {
            entry_type,
            timestamp,
            guard,
        })
    }
}

fn main() {
    let start = Instant::now();
    let file = fs::read_to_string("./src/input").unwrap();
    let mut entries: Vec<Entry> = file.lines().map(|l| l.parse().unwrap()).collect();
    entries.sort_by(|a, b| (&a.timestamp).partial_cmp(&b.timestamp).unwrap());

    let sleep_log = parse_logs(entries);

    let (sleepiest_guard, sleepiest_minute) = part_1(&sleep_log);

    println!(
        "Part 1: {} (Guard {} sleeps most at minute {})",
        sleepiest_guard * sleepiest_minute,
        sleepiest_guard,
        sleepiest_minute
    );

    let (guard, most_common_minute) = part_2(&sleep_log);

    println!(
        "Part 2: {} (Guard {} slept more than any guard on the same minute, {})",
        &guard * &most_common_minute,
        &guard,
        &most_common_minute
    );

    println!("{}ms", start.elapsed().as_millis());
}

fn part_1(sleep_log: &HashMap<String, Vec<u32>>) -> (u32, u32) {
    let sleepiest_guard = sleep_log
        .iter()
        .max_by(|a, b| a.1.len().cmp(&b.1.len())) // Guard with most minutes slept
        .map(|(k, _v)| k)
        .unwrap();

    let (sleepiest_minute, _) = get_most_common_minute(sleep_log.get(sleepiest_guard).unwrap());
    (sleepiest_guard.parse().unwrap(), sleepiest_minute)
}

fn part_2(sleep_log: &HashMap<String, Vec<u32>>) -> (u32, u32) {
    let guards_most_common_minutes: Vec<(&String, (u32, u32))> = sleep_log
        .iter()
        .map(|(k, v)| (k, get_most_common_minute(v)))
        .collect();

    guards_most_common_minutes
        .iter()
        .max_by(|a, b| a.1 .1.cmp(&b.1 .1)) // Guard with the most time on any minute
        .map(|(guard, (min, _count))| (guard.parse().unwrap(), *min)) // Get the minute
        .unwrap()
}

fn parse_logs(entries: Vec<Entry>) -> HashMap<String, Vec<u32>> {
    let mut sleep_log: HashMap<String, Vec<u32>> = HashMap::new();
    let mut sleeping: String = String::new();
    let mut since: u32 = 0;

    // Parse the logs, entry-by-
    for entry in entries {
        match entry.entry_type {
            EntryType::Rotate => sleeping = entry.guard.unwrap(),
            EntryType::Sleep => since = entry.timestamp.minute(),
            EntryType::Wake => fill_log(since, entry.timestamp.minute(), &sleeping, &mut sleep_log),
        }
    }

    sleep_log
}

fn get_most_common_minute(minutes: &Vec<u32>) -> (u32, u32) {
    let mut counts: HashMap<u32, u32> = HashMap::new();

    for minute in minutes {
        let entry = counts.entry(*minute).or_insert(0);
        *entry += 1;
    }

    counts
        .iter()
        .max_by(|a, b| a.1.cmp(b.1))
        .map(|(k, v)| (*k, *v))
        .unwrap()
}

fn fill_log(start: u32, stop: u32, guard: &String, log: &mut HashMap<String, Vec<u32>>) {
    let map_ref = log.entry(String::from(guard)).or_insert(vec![]);

    // Fill the sleep log for this guard from minute [start, stop)
    map_ref.extend(start..stop);
}
