use std::{
    borrow::Borrow,
    collections::{HashMap, HashSet},
    ops::Sub,
    str::FromStr,
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
    signals.sort_by(|a, b| a.len().cmp(&b.len()));

    for bad_signal in signals {
        for match_ in number_segments
            .iter()
            .filter(|s| s.len() == bad_signal.len())
        {
            for section in match_ {
                if !possible_segment_wires.contains_key(section) {
                    possible_segment_wires.insert(*section, HashSet::from_iter(bad_signal.chars()));
                }
            }
        }
    }

    dbg!(&possible_segment_wires);
    // dbg!(filtered_segment_wires);
    while possible_segment_wires
        .values()
        .map(|v| v.len())
        .sum::<usize>()
        > 9
    {
        let mut choices: Vec<(char, usize)> = possible_segment_wires
            .iter()
            .map(|(k, v)| (*k, v.len()))
            .collect();
        choices.sort_by(|a, b| a.1.cmp(&b.1));

        for (wire, _) in choices {
            possible_segment_wires = filter_out_choices(wire, &mut possible_segment_wires);
        }
        dbg!(&possible_segment_wires);
    }

    output_digits
}

fn filter_out_choices(
    wire: char,
    possible_choices: &mut HashMap<char, HashSet<char>>,
) -> HashMap<char, HashSet<char>> {
    let mut filtered_segment_wires: HashMap<char, HashSet<char>> = possible_choices.clone();
    let choices = possible_choices.get(&wire).unwrap();

    for (other_wire, other_choices) in possible_choices.iter() {
        if wire == *other_wire || choices == other_choices {
            continue;
        }
        let diff = other_choices - choices;
        filtered_segment_wires.insert(*other_wire, diff);
    }

    filtered_segment_wires
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
