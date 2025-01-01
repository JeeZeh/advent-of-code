use std::{cmp::Ordering, collections::HashMap};

use advent_of_code::lines_no_empty;
use itertools::Itertools;

advent_of_code::solution!(5);

pub fn solve(input: &str) -> (Option<u64>, Option<u64>) {
    let (top, bottom) = input.split_once("\n\n").unwrap();
    let mut rules: [u128; 100] = [0; 100];
    lines_no_empty(top)
        .map(|l| l.split_once("|").unwrap())
        .for_each(|(l, r)| rules[l.parse::<usize>().unwrap()] += 1 << r.parse::<u8>().unwrap());

    let mut valid_mids = Vec::new();
    let mut invalid_mids = Vec::new();
    lines_no_empty(bottom)
        .map(|l| l.split(",").map(|l| l.parse().unwrap()).collect_vec())
        .map(|update| {
            let sorted = update
                .iter()
                .copied()
                .sorted_by(|&a, &b| sorted_rule(&rules, a, b))
                // TODO(optimization): Remove collect().
                .collect_vec();

            match sorted == update {
                true => valid_mids.push(update[update.len() / 2] as u64),
                false => invalid_mids.push(sorted[sorted.len() / 2] as u64),
            }
        })
        .collect_vec();

    (
        Some(valid_mids.iter().sum()),
        Some(invalid_mids.iter().sum()),
    )
}

fn sorted_rule(rules: &[u128], a: u8, b: u8) -> Ordering {
    if a == b {
        return Ordering::Equal;
    }
    if rules[a as usize] & (1 << &(b as u128)) != 0 {
        return Ordering::Less;
    }
    return Ordering::Greater;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let result = solve(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, (Some(143), Some(123)));
    }
}
