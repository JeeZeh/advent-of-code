use std::collections::HashMap;

use itertools::Itertools;

pub fn solve(lines: String) -> (i64, i64) {
    let mut parts = lines.split("\n\n");

    // Parse polymer into count of pairs
    let mut pairs: HashMap<(char, char), i64> = HashMap::new();
    let polymer = parts.next().unwrap().chars().collect_vec();
    for chars in polymer.windows(2) {
        let pair = (chars[0], chars[1]);
        let entry = pairs.entry(pair).or_insert(0);
        *entry += 1;
    }

    // Init count map
    let mut count: HashMap<char, i64> = HashMap::new();
    for char in polymer {
        *count.entry(char).or_insert(0) += 1;
    }

    // Parse pair templates
    let templates: HashMap<(char, char), char> =
        parts.next().unwrap().lines().map(parse_template).collect();

    for _ in 0..10 {
        pairs = step(&pairs, &templates, &mut count);
    }

    let part_one = count.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap().1
        - count.iter().min_by(|a, b| a.1.cmp(&b.1)).unwrap().1;

    for _ in 10..40 {
        pairs = step(&pairs, &templates, &mut count);
    }

    let part_two = count.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap().1
        - count.iter().min_by(|a, b| a.1.cmp(&b.1)).unwrap().1;

    (part_one, part_two)
}

fn step(
    pairs: &HashMap<(char, char), i64>,
    templates: &HashMap<(char, char), char>,
    counts: &mut HashMap<char, i64>,
) -> HashMap<(char, char), i64> {
    let mut new_pairs = HashMap::new();

    for (pair, count) in pairs {
        if let Some(output) = templates.get(pair) {
            if *count == 0 {
                continue;
            }

            // Increment new pattern counts
            *new_pairs.entry((pair.0, *output)).or_insert(0) += count;
            *new_pairs.entry((*output, pair.1)).or_insert(0) += count;

            // Increment the count for the newly inserted char
            // dbg!(template, output, existing_entry);
            *counts.entry(*output).or_insert(0) += *count;
        }
    }

    new_pairs
}

// fn simulated_part_one(lines: &String) -> i64 {
//     let mut parts = lines.split("\n\n");
//     let mut polymer = parts.next().unwrap().chars().collect_vec();

//     let templates: HashMap<(char, char), char> =
//         parts.next().unwrap().lines().map(parse_template).collect();

//     for _ in 0..10 {
//         polymer = simulated_step(&polymer, &templates);
//     }

//     0
// }

// pub fn simulated_step(init: &Vec<char>, templates: &HashMap<(char, char), char>) -> Vec<char> {
//     let mut new_polymer = Vec::with_capacity((init.len() * 2) - 1);

//     for w in init.windows(2) {
//         new_polymer.push(w[0]);
//         if let Some(template_match) = templates.get(w) {
//             new_polymer.push(*template_match);
//         }
//         new_polymer.push(w[1]);
//     }

//     new_polymer
// }

fn parse_template(line: &str) -> ((char, char), char) {
    let mut parts = line.split(" -> ");
    let left = parts.next().unwrap().chars().collect_tuple().unwrap();
    let right = parts.next().unwrap().chars().collect_vec();
    (left, right[0])
}
