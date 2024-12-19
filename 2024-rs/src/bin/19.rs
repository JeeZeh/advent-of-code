use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(19);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

fn find_configurations<'a>(
    towels: &[Vec<Color>],
    to_make: &'a [Color],
    memo: &mut HashMap<&'a [Color], usize>,
) -> usize {
    if to_make.is_empty() {
        return 1;
    }

    if let Some(&cached) = memo.get(to_make) {
        return cached;
    }

    let mut configurations = 0;
    for towel in towels {
        if to_make.starts_with(towel) {
            configurations += find_configurations(towels, &to_make[towel.len()..], memo);
        }
    }
    memo.insert(to_make, configurations);
    configurations
}

// HINT USED: Confirmed that memoization would work.
pub fn solve(input: &str) -> (Option<usize>, Option<usize>) {
    let (towels, patterns) = parse_input(input);

    let mut memo = HashMap::new();
    let possible_configurations = patterns
        .iter()
        .map(|p| find_configurations(&towels, p, &mut memo))
        .collect_vec();

    (
        Some(possible_configurations.iter().filter(|&&c| c > 0).count()),
        Some(possible_configurations.iter().sum()),
    )
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
        let result: (Option<usize>, Option<usize>) =
            solve(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, (Some(6), Some(16)));
    }
}
