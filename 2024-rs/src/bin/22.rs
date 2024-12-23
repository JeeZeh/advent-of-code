#![feature(int_roundings)]
#![feature(iter_map_windows)]
use std::{num::ParseIntError, str::FromStr};

use advent_of_code::lines_no_empty;
use itertools::Itertools;

advent_of_code::solution!(22);

struct Buyer {
    secret: u32,
}

impl FromStr for Buyer {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<u32>() {
            Ok(secret) => Ok(Buyer { secret }),
            Err(e) => Err(e),
        }
    }
}

impl Buyer {
    fn evolve(&mut self) -> &mut Buyer {
        self.secret = (self.secret ^ (self.secret * 64)) % 16777216;
        self.secret = (self.secret ^ self.secret.div_floor(32)) % 16777216;
        self.secret = (self.secret ^ (self.secret * 2048)) % 16777216;

        self
    }
}

pub fn solve(input: &str) -> (Option<u64>, Option<u64>) {
    let mut buyers = lines_no_empty(input)
        .map(|l| Buyer::from_str(l).unwrap())
        .collect_vec();

    // Very simple vectorization: https://www.reddit.com/r/adventofcode/comments/1hjroap/2024_day_22_solutions/m39nlbn/
    let mut costs = vec![0; 0xFFFFF];
    let mut seen = vec![0; 0xFFFFF];

    for buyer in buyers.iter_mut() {
        let original = buyer.secret;
        let mut result = buyer.secret;
        let mut previous_cost = result % 10;
        let mut deltas = 0;

        for i in 0..2000 {
            result = buyer.evolve().secret;
            let cost = result % 10;

            // offset cost delta by +10 and represent as 5 bit unsigned int (max == 19 == 0b10101)
            // store sliding window of 4 deltas as a 20 bit unsigned int (max == 0xFFFFF)
            deltas = ((deltas << 5) & 0xFFFFF) + 10 + cost - previous_cost;

            // start checking prices once deltas window is populated
            // only counting the first occurance of each unique delta sequence
            if seen[deltas as usize] != original && i >= 3 {
                seen[deltas as usize] = original;
                costs[deltas as usize] += cost;
            }

            previous_cost = cost;
        }
    }

    let sum_2000 = buyers.iter().map(|b| b.secret as u64).sum::<u64>();
    let &best_sequence = costs.iter().max().unwrap();

    (Some(sum_2000), Some(best_sequence as u64))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_1() {
        let result = solve(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, (Some(37327623), Some(24)));
    }

    #[test]
    fn test_solve_2() {
        let result = solve(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, (Some(37990510), Some(23)));
    }
}
