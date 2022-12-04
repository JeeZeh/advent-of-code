use std::collections::{HashMap, HashSet};

use itertools::Itertools;

fn get_common_badges(input: &[String]) -> Vec<char> {
    let mut common_badges: Vec<char> = Vec::new();

    for mut group in input
        .iter()
        .map(|s| s.chars().collect::<HashSet<char>>())
        .chunks(3)
        .into_iter()
    {
        // Needed because multi-set intersection is not possible
        let mut first: HashSet<char> = group.next().unwrap();
        while let Some(other) = group.next() {
            first.retain(|e| other.contains(e));
        }
        common_badges.push(*first.iter().next().unwrap());
    }

    common_badges
}

fn get_common_items(input: &[String]) -> Vec<char> {
    input
        .iter()
        .map(|l| {
            (
                l[..l.len() / 2].chars().collect::<HashSet<char>>(),
                l[l.len() / 2..].chars().collect::<HashSet<char>>(),
            )
        })
        .map(|(left, right)| *left.intersection(&right).next().unwrap())
        .collect_vec()
}

fn build_item_values() -> HashMap<char, usize> {
    let mut item_values: HashMap<char, usize> = HashMap::new();
    for (i, c) in "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .chars()
        .enumerate()
    {
        item_values.insert(c, i + 1);
    }
    item_values
}

pub fn solve(input: Vec<String>) -> (usize, usize) {
    let item_values = build_item_values();

    (
        get_common_items(&input)
            .iter()
            .map(|c| item_values.get(c).unwrap())
            .sum(),
        get_common_badges(&input)
            .iter()
            .map(|c| item_values.get(c).unwrap())
            .sum(),
    )
}
