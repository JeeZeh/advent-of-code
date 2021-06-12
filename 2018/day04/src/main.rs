use chrono::prelude::*;
use std::{fs, str::FromStr, string::ParseError};


enum EntryType {
    Wake,
    Sleep,
    Rotate,
}
struct Entry {
    entry_type: EntryType,
    timestamp: DateTime<Utc>,
    guard: Option<String>
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
        let guard: String;

        if entry.contains("wake") {
            entry_type = EntryType::Wake;
        } else if entry.contains("sleep") {
            entry_type = EntryType::Sleep;
        } else {
            entry_type = EntryType::Rotate;
            guard = String::from(&entry.split(" ").nth(1).unwrap()[1..]);
        }

        let guard = Some(guard);

        Ok(Entry { entry_type, timestamp, guard})
    }
}

fn main() {
    let file = fs::read_to_string("./src/sample").unwrap();
    let entry: Entry = file.lines().nth(0).unwrap().parse().unwrap();

    println!("{}", entry.timestamp)
}
