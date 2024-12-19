use std::{collections::VecDeque, ops::Range};

use itertools::Itertools;

advent_of_code::solution!(19);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Color {
    White,
    Blue,
    Black,
    Red,
    Green,
}

impl Color {
    fn from_char(c: char) -> Self {
        match c {
            'w' => Color::White,
            'u' => Color::Blue,
            'b' => Color::Black,
            'r' => Color::Red,
            'g' => Color::Green,
            _ => panic!("Unknown color: {c}!"),
        }
    }
}

trait Towel {}

impl Towel for Vec<Color> {}

fn find_configuration(towels: &[Vec<Color>], to_make: &[Color]) -> bool {
    let mut queue = VecDeque::new();
    queue.push_back(to_make);

    while let Some(remaining) = queue.pop_back() {
        if remaining.is_empty() {
            return true;
        }

        for towel in towels {
            if remaining.starts_with(towel) {
                queue.push_back(&remaining[towel.len()..]);
            }
        }
    }

    false
}

fn merge_overlapping_intervals(arr: &mut Vec<Range<usize>>) -> Vec<Range<usize>> {
    arr.sort_by(|a, b| a.start.cmp(&b.start));

    let mut result: Vec<Range<usize>> = Vec::new();
    result.push(arr[0].clone());

    for i in 1..arr.len() {
        let current: Range<usize> = arr[i].clone();
        let j: usize = result.len() - 1;

        if current.start >= result[j].start && current.start <= result[j].end {
            result[j].end = current.end.max(result[j].end);
        } else {
            result.push(current);
        }
    }
    result
}

fn find_configuration_v2(towels: &[Vec<Color>], to_make: &[Color]) -> bool {
    let mut ranges = Vec::new();

    for i in 0..to_make.len() {
        for towel in towels {
            if to_make[i..].starts_with(&towel) {
                ranges.push(i..i + towel.len());
            }
        }
    }

    let merged = merge_overlapping_intervals(&mut ranges);
    return merged.len() == 1 && merged[0].start == 0 && merged[0].end == to_make.len();
}

pub fn solve(input: &str) -> (Option<u64>, Option<u64>) {
    let (towels, patterns) = parse_input(input);

    let mut valid = 0;
    for pattern in &patterns {
        println!("Testing {pattern:?}");
        if find_configuration_v2(&towels, &pattern) {
            println!("Valid");
            if cfg!(debug_assertions) {}
            valid += 1;
        };
    }

    println!("{valid}/{}", &patterns.len());
    (Some(valid), None)
}

fn parse_input(input: &str) -> (Vec<Vec<Color>>, Vec<Vec<Color>>) {
    let (top, bottom) = input.split_once("\n\n").unwrap();
    let mut towels: Vec<Vec<Color>> = top
        .split(", ")
        .map(|towel| towel.chars().map(Color::from_char).collect())
        .collect();

    towels.sort_by(|a, b| b.len().cmp(&a.len()));

    if cfg!(debug_assertions) {
        println!("Towels: {towels:?}");
    }

    let patterns: Vec<Vec<Color>> = bottom
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().map(Color::from_char).collect())
        .collect();
    if cfg!(debug_assertions) {
        println!("Patterns: {patterns:?}");
    }

    (towels, patterns)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_solve() {
        let result = solve(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, (Some(6), None));
    }

    #[test]
    fn test_invalid() {
        let towels = vec![
            vec![Color::Black, Color::White, Color::Blue],
            vec![Color::White, Color::Red],
            vec![Color::Red, Color::Black],
            vec![Color::Green, Color::Black],
            vec![Color::Black, Color::Red],
            vec![Color::Red],
            vec![Color::Black],
            vec![Color::Green],
        ];

        let pattern = vec![Color::Blue, Color::Black, Color::White, Color::Blue];
        assert_eq!(find_configuration_v2(&towels, &pattern), false);
    }
}
