use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use itertools::Itertools;

pub fn solve(lines: Vec<String>) -> (usize, usize) {
    let parsed: Vec<(Vec<String>, Vec<String>)> = lines.iter().map(|l| parse_input(l)).collect();
    let mut digits: Vec<usize> = Vec::new();

    for (inputs, outputs) in parsed {
        digits.append(&mut solve_line(&inputs, &outputs))
    }

    let part_one = digits.iter().filter(|d| [1, 4, 7, 8].contains(d)).count();
    let part_two = digits
        .windows(4)
        .map(|window| {
            window
                .iter()
                .map(|d| format!("{}", d))
                .collect::<String>()
                .parse::<usize>()
                .unwrap()
        })
        .sum();

    (part_one, part_two)
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

    signals.sort_by(|a, b| b.len().cmp(&a.len()));

    // Build a mapping of correct segments to choices of messed up segments
    for bad_segment in &signals {
        let segs = HashSet::from_iter(bad_segment.chars());
        for match_ in number_segments
            .iter()
            .filter(|s| s.len() == bad_segment.len())
        {
            for section in match_ {
                possible_segment_wires.insert(*section, segs.clone());

                // Can't figure out how to not do this for each char
                possible_segment_wires = filter_out_choices(*section, &mut possible_segment_wires);
            }
        }
    }

    // The digit segments we've yet to identify
    let mut to_identify: Vec<String> = signals
        .iter()
        .map(|s| s.chars().sorted().collect())
        .collect();

    // The digit segments we've found so far
    let mut known: HashMap<String, usize> = HashMap::new();

    // This is the only "brute force" section, though it's not random.
    // Here we try to find a mapping for all digits. We try every digit repeatedly until
    // we've found them all, which is guaranteed though may take a couple of passes:
    // E.g. It may be the case that we can not identify '5' since whatever substring
    // 'find_digit_mapping' is working with is shared with another digit, e.g. '6'
    // Eventually, we will correctly identify '6', and so '5' can be found on the next pass.
    while known.len() < 9 {
        for d in 0..10 {
            if let Some(found) =
                find_digit_mapping(&number_segments[d], &possible_segment_wires, &to_identify)
            {
                known.insert(found.clone(), d);
                to_identify.retain(|s| s != &found);
            }
        }
    }

    // Finally, convert the 4 output signals we received to their actual digits
    // by using the mappings we've identified
    for output in outputs {
        let digit_string: String = output.chars().sorted().collect();
        output_digits.push(*known.get(&digit_string).unwrap());
    }

    output_digits
}

fn find_digit_mapping(
    digit_segments: &HashSet<char>,
    possible_segment_wires: &HashMap<char, HashSet<char>>,
    signals_to_identify: &Vec<String>,
) -> Option<String> {
    let mut options: HashMap<char, usize> = HashMap::new();

    // Collect the number of times each possible char appears when trying
    // to construct the given digit_segments
    // If for a given segment, there is a choice of multiple chars, keep track
    // of these choices
    // If the same choice appears 2 times over the whole digit (its a choice for 2 segments)
    // it means it *must* appear in the digit somewhere (either of the two)
    for seg in digit_segments {
        let choices = possible_segment_wires.get(&seg).unwrap();
        if choices.len() == 1 {
            let definite = **choices.iter().collect::<Vec<&char>>().first().unwrap();
            options.insert(definite, 2); // Hard code it as 2 since we know it appears for sure
        } else {
            for choice in choices {
                let entry = options.entry(*choice).or_insert(0);
                *entry += 1;
            }
        }
    }

    // With the choices collected, we can determine which chars must appear
    // in the segment signal by filtering for counts of 2
    let known_chars: String = options
        .iter()
        .filter(|(_, v)| **v == 2)
        .map(|(c, _)| *c)
        .sorted()
        .collect();

    // Try and find a matching signal with our known substring and length
    let signal_match: Vec<&String> = signals_to_identify
        .iter()
        .filter(|s| s.len() == digit_segments.len() && known_chars.chars().all(|c| s.contains(c)))
        .collect();

    // If we find one, great.
    if signal_match.len() == 1 {
        Some(signal_match[0].clone())
    } else {
        None
    }
}

/**
 * For a given wire, tries to reduce the set of possibilities by eliminating
 * its mappings from any other wire's mappings
 */
fn filter_out_choices(
    wire: char,
    possible_choices: &mut HashMap<char, HashSet<char>>,
) -> HashMap<char, HashSet<char>> {
    let mut filtered_segment_wires: HashMap<char, HashSet<char>> = possible_choices.clone();
    let choices = possible_choices.get(&wire).unwrap();

    for (other_wire, other_choices) in possible_choices.iter() {
        if wire == *other_wire || choices == other_choices || choices.len() >= other_choices.len() {
            continue;
        }
        let intersection: HashSet<char> = choices
            .intersection(other_choices)
            .into_iter()
            .cloned()
            .collect();

        filtered_segment_wires.insert(*other_wire, other_choices - &intersection);
    }

    filtered_segment_wires
}

fn parse_input(line: &str) -> (Vec<String>, Vec<String>) {
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
