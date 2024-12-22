#![feature(int_roundings)]
#![feature(iter_map_windows)]
use std::{collections::HashMap, num::ParseIntError, str::FromStr};

use advent_of_code::lines_no_empty;
use itertools::Itertools;

advent_of_code::solution!(22);

struct Buyer {
    secret: u64,
}

impl FromStr for Buyer {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<u64>() {
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

    let mut sequence_map: HashMap<(i8, i8, i8, i8), u64> = HashMap::new();

    for buyer in buyers.iter_mut() {
        (0..2000)
            .map(|_| {
                let old_one = (buyer.secret % 10) as i8;
                let new_one = (buyer.evolve().secret % 10) as i8;
                ((old_one - new_one), new_one)
            })
            .map_windows(|[(a, _), (b, _), (c, _), (d, val)]| ((*a, *b, *c, *d), *val))
            .unique_by(|a| a.0)
            .for_each(|(seq, val)| *sequence_map.entry(seq).or_default() += val as u64);
    }
    let sum_2000: u64 = buyers.iter().map(|b| b.secret).sum();
    let &best_sequence = sequence_map.values().max().unwrap();

    (Some(sum_2000), Some(best_sequence))
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
