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

    let mut hashed = vec![0; 0xFFFFF];

    for buyer in buyers.iter_mut() {
        (0..2000)
            .map(|_| {
                let old_one = (buyer.secret % 10) as i8;
                let new_one = (buyer.evolve().secret % 10) as i8;
                ((old_one - new_one), new_one)
            })
            .map_windows(|[(a, _), (b, _), (c, _), (d, val)]| (hash(*a, *b, *c, *d), *val))
            .unique_by(|f| f.0)
            .for_each(|(seq, val)| hashed[seq as usize] += val as u64);
    }
    let sum_2000: u64 = buyers.iter().map(|b| b.secret).sum();
    let &best_sequence = hashed.iter().max().unwrap();

    (Some(sum_2000), Some(best_sequence))
}

fn hash(a: i8, b: i8, c: i8, d: i8) -> u32 {
    (((a as u32 + 10) << 15) + ((b as u32 + 10) << 10) + ((c as u32 + 10) << 5) + (d as u32 + 10))
        & 0xFFFFF
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
