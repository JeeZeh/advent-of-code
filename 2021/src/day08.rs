use std::{
    borrow::Borrow,
    collections::{HashMap, HashSet},
    str::FromStr, ops::Sub,
};

pub fn solve(lines: Vec<String>) -> (usize, usize) {
    let parsed: Vec<(Vec<String>, Vec<String>)> = lines.iter().map(|l| parse(l)).collect();

    solve_line(&parsed[0].0, &parsed[0].1);

    (0, 0)
}

fn solve_line(signals: &[String], outputs: &[String]) -> Vec<usize> {
    let mut signals = signals.to_owned();
    let mut output_digits = Vec::new();
    let mut possible_segment_wires: HashMap<char, HashSet<char>> = HashMap::new();

    let number_segments: Vec<HashSet<char>> = [
        "abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg",
    ]
    .iter()
    .map(|s| HashSet::from_iter(s.chars()))
    .collect();

    dbg!(&possible_segment_wires);

    // Reverse sort to start with the largest
    signals.sort_by(|a, b| b.len().cmp(&a.len()));

    for bad_signal in signals {
        for match_ in number_segments
            .iter()
            .filter(|s| s.len() == bad_signal.len())
        {
            for section in match_ {
                possible_segment_wires
                    .entry(*section)
                    .or_insert(HashSet::from_iter(bad_signal.chars()));
            }

            for mismatch in number_segments[8].sub(&HashSet::from_iter(bad_signal.chars())) {
                possible_segment_wires.entry(*mismatch)
            };

            for mismatch in number_segments
                .iter()
                .filter(|s| s.len() > bad_signal.len())
            {
                for section in mismatch {
                    possible_segment_wires.entry(*section).and_modify(|e| {
                        bad_signal.chars().for_each(|c| {
                            if *section == c {
                                e.remove(&c);
                            }
                        });
                    });
                }
            }
        }
    }

    dbg!(possible_segment_wires);

    output_digits
}

fn build_possible_segments_map() -> HashMap<char, Vec<char>> {
    let mut possible_segment_wires: HashMap<char, Vec<char>> = HashMap::new();
    let segments = Vec::from_iter("abcdefg".chars());
    for char in &segments {
        possible_segment_wires.insert(*char, segments.clone());
    }
    possible_segment_wires
}

fn parse(line: &str) -> (Vec<String>, Vec<String>) {
    let mut parts = line.split(" | ");

    let signals = parts
        .next()
        .unwrap()
        .split(' ')
        .map(|p| String::from_str(p).unwrap())
        .collect();
    let outputs = parts
        .next()
        .unwrap()
        .split(' ')
        .map(|p| String::from_str(p).unwrap())
        .collect();

    (signals, outputs)
}
