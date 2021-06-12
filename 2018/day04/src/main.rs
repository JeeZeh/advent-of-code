use chrono::prelude::*;
use std::{fs, str::FromStr, string::ParseError};

#[derive(Debug)]

enum EntryType {
    Wake,
    Sleep,
    Rotate,
}

#[derive(Debug)]
struct Entry {
    entry_type: EntryType,
    timestamp: DateTime<Utc>,
    guard: Option<String>,
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
    let file = fs::read_to_string("./src/sample").unwrap();
    let mut entries: Vec<Entry> = file.lines().map(|l| l.parse().unwrap()).collect();
    entries.sort_by(|a, b| (&a.timestamp).partial_cmp(&b.timestamp).unwrap());
    entries.iter().for_each(|f| println!("{:?}", f));
}
