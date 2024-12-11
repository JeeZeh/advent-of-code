use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(11);

// Initial idea was to use a counter of occurrences of each number but thought part two would
// ask for 1000s of blinks and I'd have too many unique numbers to be fast. Instead, I figured
// I needed to do something with caching the number of stones produced by N blinks of a given stone.
// HINT USED: confirmed on Reddit before implementing that this would work.
fn get_stones(num: u64, blinks_left: u8, cache: &mut HashMap<(u64, u8), u64>) -> u64 {
    if blinks_left == 0 {
        return 1;
    }

    if let Some(cached) = cache.get(&(num, blinks_left)) {
        return *cached;
    }

    let mut stones = 0;
    if num == 0 {
        stones += get_stones(1, blinks_left - 1, cache);
    } else {
        let as_str = num.to_string();
        if as_str.len().rem_euclid(2) == 0 {
            let left = &as_str[0..as_str.len() / 2].parse::<u64>().unwrap();
            let right = &as_str[as_str.len() / 2..].parse::<u64>().unwrap();
            stones += get_stones(*left, blinks_left - 1, cache);
            stones += get_stones(*right, blinks_left - 1, cache);
        } else {
            stones += get_stones(num * 2024, blinks_left - 1, cache);
        }
    }

    cache.insert((num, blinks_left), stones);
    stones
}

pub fn solve(input: &str) -> (Option<u64>, Option<u64>) {
    let initial = input
        .split(" ")
        .map(|n| n.parse::<u64>().unwrap())
        .collect_vec();

    let mut cache: HashMap<(u64, u8), u64> = HashMap::new();
    let part_one = initial
        .iter()
        .map(|stone| get_stones(*stone, 25, &mut cache))
        .sum::<u64>();

    let part_two = initial
        .iter()
        .map(|stone| get_stones(*stone, 75, &mut cache))
        .sum::<u64>();

    (Some(part_one), Some(part_two))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let result = solve(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, (Some(55312), None));
    }
}
