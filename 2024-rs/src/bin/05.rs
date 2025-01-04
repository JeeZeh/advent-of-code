#![feature(iter_collect_into)]
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

    let mut valid_mids: u64 = 0;
    let mut invalid_mids: u64 = 0;
    for line in lines_no_empty(bottom) {
        // Manually fill an array instead of allocating a Vec<T> every time
        let mut stable_vec: [u8; 100] = [0; 100];
        let mut ptr = 0;
        let mut iter = line.split(",").map(|c| c.parse::<u8>());
        while ptr < 100 {
            if let Some(Ok(next)) = iter.next() {
                stable_vec[ptr] = next;
                ptr += 1;
            } else {
                break;
            }
        }

        let mid = ptr / 2;
        let sorted = stable_vec[..ptr]
            .iter()
            .sorted_by(|&&a, &&b| sorted_rule(&rules, a, b));
        let mut is_sorted = true;
        let mut mid_val = 0;
        for (i, (&orig, &sort)) in stable_vec[..ptr].iter().zip_eq(sorted).enumerate() {
            if orig != sort {
                is_sorted = false;
                // If we discover this needs to be fixed after passing the midpoint, we already know its value.
                if i > mid {
                    break;
                }
            }
            if i == mid {
                mid_val = sort;
                // If we already know this needs to be fixed, we can immediately return the mid-value.
                if !is_sorted {
                    break;
                }
            }
        }
        if is_sorted {
            valid_mids += mid_val as u64;
        } else {
            invalid_mids += mid_val as u64;
        }
    }

    (Some(valid_mids), Some(invalid_mids))
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
