use ahash::AHashMap;
use itertools::Itertools;

pub fn solve(lines: String) -> (u64, u64) {
    let mut parts = lines.split("\n\n");

    // Parse polymer into count of pairs
    let mut pairs: AHashMap<(char, char), u64> = AHashMap::new();
    let polymer = parts.next().unwrap().chars().collect_vec();
    for chars in polymer.windows(2) {
        let pair = (chars[0], chars[1]);
        let entry = pairs.entry(pair).or_insert(0);
        *entry += 1;
    }

    // Init count map
    let mut count: AHashMap<char, u64> = AHashMap::new();
    for char in polymer {
        *count.entry(char).or_insert(0) += 1;
    }

    // Parse pair templates
    let templates: AHashMap<(char, char), char> =
        parts.next().unwrap().lines().map(parse_template).collect();

    // Part 1
    for _ in 0..10 {
        pairs = step(&pairs, &templates, &mut count);
    }
    let part_one = count.values().max().unwrap() - count.values().min().unwrap();

    // Part 2
    for _ in 10..40 {
        pairs = step(&pairs, &templates, &mut count);
    }
    let part_two = count.values().max().unwrap() - count.values().min().unwrap();

    (part_one, part_two)
}

/**
 * Since we can describe a template process AB -> C as producing two pairs AC and CB,
 * we can just keep track of these changes. So if we know that we have 2 occurences of AB,
 * after the step we will have 2 occurrences of AC and 2 of CB. We discard the original counts
 * of AB in the process.
 */
fn step(
    pairs: &AHashMap<(char, char), u64>,
    templates: &AHashMap<(char, char), char>,
    counts: &mut AHashMap<char, u64>,
) -> AHashMap<(char, char), u64> {
    let mut new_pairs = AHashMap::new();

    for (pair, count) in pairs {
        if let Some(output) = templates.get(pair) {
            // Increment new pattern counts
            *new_pairs.entry((pair.0, *output)).or_insert(0) += count;
            *new_pairs.entry((*output, pair.1)).or_insert(0) += count;

            // Increment the count for the newly inserted char
            // We neex to count as part of each step, otherwise we
            // can't tell at the end which characters overlap between patterns
            *counts.entry(*output).or_insert(0) += *count;
        }
    }

    new_pairs
}

fn parse_template(line: &str) -> ((char, char), char) {
    let mut parts = line.split(" -> ");
    let left = parts.next().unwrap().chars().collect_tuple().unwrap();
    let right = parts.next().unwrap().chars().collect_vec();
    (left, right[0])
}
