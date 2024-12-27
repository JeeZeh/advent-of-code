#![feature(iter_map_windows)]
use advent_of_code::lines_no_empty;
use itertools::Itertools;

advent_of_code::solution!(2);

fn is_safe(levels: &[i32]) -> bool {
    let sequence = levels
        .iter()
        .map_windows(|[a, b]| (*b - *a))
        .filter(|i| i.abs() <= 3)
        .map(|a| a.signum())
        .collect_vec();
    sequence.iter().sum::<i32>().abs() == (levels.len() as i32) - 1
}

fn can_make_safe(levels: &[i32]) -> bool {
    let dir = levels
        .iter()
        .map_windows(|[a, b]| (*b - *a).signum())
        .sum::<i32>()
        .signum();

    // Inconclusive direction
    if dir == 0 {
        return false;
    }

    levels
        .iter()
        .map_windows(|&[a, b]| b - a)
        .enumerate()
        .all(|(i, diff)| {
            // Either the difference is valid (in the common direction and less than 3 in magnitude)
            (diff.signum() == dir && diff.abs() <= 3)
            // Or removing either side of the difference fixes it.
            || is_safe(&to_vec_without(levels, i))
            // We know that if this fails for any pair, we will need to remove more than 1 element.
                || is_safe(&to_vec_without(levels, i + 1))
        })
}

fn to_vec_without(levels: &[i32], to_remove: usize) -> Vec<i32> {
    let new_slice: Vec<i32>;
    if to_remove == 0 {
        new_slice = levels[1..].to_vec();
    } else if to_remove == levels.len() - 1 {
        new_slice = levels[..to_remove].to_vec();
    } else {
        new_slice = [&levels[..to_remove], &levels[to_remove + 1..]].concat();
    }
    new_slice
}

pub fn solve(input: &str) -> (Option<u64>, Option<u64>) {
    let (original_safe, original_unsafe): (Vec<_>, Vec<_>) = lines_no_empty(input)
        .map(|l| l.split(" ").map(|c| c.parse().unwrap()).collect_vec())
        .partition(|levels| is_safe(levels));
    (
        Some(original_safe.len() as u64),
        Some(
            (original_safe.len()
                + original_unsafe
                    .iter()
                    .filter(|unsafe_| can_make_safe(&unsafe_))
                    .count()) as u64,
        ),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let result = solve(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, (Some(2), Some(4)));
    }
}
