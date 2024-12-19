#![feature(iter_map_windows)]
use itertools::Itertools;

advent_of_code::solution!(2);

fn is_safe(levels: &[i32]) -> bool {
    let sequence = levels
        .iter()
        .filter(|a| **a != 0 && a.abs() <= 3)
        .map_windows(|[a, b]| (*b - *a).min(-1).max(1))
        .collect_vec();
    sequence.len() == levels.len() && sequence.iter().unique().try_len().unwrap() == levels.len()
}

pub fn solve(input: &str) -> (Option<u64>, Option<u64>) {
    let lines: Vec<Vec<i32>> = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.split(" ").map(|c| c.parse().unwrap()).collect_vec())
        .collect();
    let original_safe = lines.iter().filter(|levels| is_safe(levels)).count();
    (Some(original_safe as u64), None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let result = solve(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, (Some(2), None));
    }
}
